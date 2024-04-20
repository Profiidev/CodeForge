use std::{
  io::{self, BufRead, BufReader, Error, Read, Write},
  process::Command,
  sync::{Arc, Condvar, Mutex},
  thread::{self, sleep},
};

use lsp_types::{
  notification::{Initialized, Notification as LSPNotificationTrait},
  request::{Initialize, Request as LSPRequestTrait},
  ClientCapabilities, InitializeParams, NumberOrString, WorkspaceFolder,
};

use crate::lsp::response::{LSPMessage, LSPResponse};

use self::{
  notification::LSPNotification,
  request::{LSPRequest, PendingRequest},
};

pub(crate) mod notification;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

#[derive(Debug)]
pub(crate) struct LSPClient {
  stdin: std::process::ChildStdin,
  pending: Vec<PendingRequest>,
}

#[derive(Debug)]
pub(crate) struct LSPClientBuilder {
  stdin: std::process::ChildStdin,
  stdout: std::process::ChildStdout,
  working_token: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LSP {
  pub lsp_client: Arc<Mutex<LSPClient>>,
}

impl LSPClient {
  pub(crate) fn create(path: String) -> Result<LSPClientBuilder, Error> {
    let mut lsp = Command::new(path)
      .stdin(std::process::Stdio::piped())
      .stdout(std::process::Stdio::piped())
      .spawn()?;

    let stdin = lsp.stdin.take().ok_or(Error::new(
      std::io::ErrorKind::Other,
      "failed to open stdin",
    ))?;

    let stdout = lsp.stdout.take().ok_or(Error::new(
      std::io::ErrorKind::Other,
      "failed to open stdout",
    ))?;

    let working_token = String::from("working_token");

    Ok(LSPClientBuilder {
      stdin,
      stdout,
      working_token,
    })
  }

  pub(crate) fn send_req<T>(&mut self, req: LSPRequest<T>) -> Result<(), Error>
  where
    T: LSPRequestTrait,
  {
    self.stdin.write(req.as_bytes()?.as_slice())?;
    self.stdin.flush()?;
    Ok(())
  }

  pub(crate) fn resolve_pending(&mut self, id: i32, body: String) {
    if let Some(request) = self.pending.iter_mut().find(|p| p.id == id) {
      request.response = Some(body);

      let (lock, cvar) = &*request.condition;
      let mut started = lock.lock().unwrap();
      *started = true;
      cvar.notify_one();
    }
  }

  pub(crate) fn send_not<T>(&mut self, not: LSPNotification<T>) -> Result<(), Error>
  where
    T: LSPNotificationTrait,
  {
    self.stdin.write(not.as_bytes()?.as_slice())?;
    self.stdin.flush()?;
    Ok(())
  }
}

impl LSPClientBuilder {
  pub(crate) async fn build(
    self,
    workspace_folders: Vec<WorkspaceFolder>,
    capabilities: ClientCapabilities,
  ) -> Result<LSP, Error> {
    let lsp_client = LSPClient {
      stdin: self.stdin,
      pending: Vec::new(),
    };

    let lsp = LSP {
      lsp_client: Arc::new(Mutex::new(lsp_client)),
    };

    let lsp_client = lsp.lsp_client.clone();
    thread::spawn(move || {
      let mut reader = BufReader::new(self.stdout);

      while let Ok(Some(msg_string)) = read_msg(&mut reader) {
        let msg: LSPMessage = serde_json::from_str(&msg_string).unwrap();
        let mut lsp = lsp_client.lock().unwrap();

        if msg.is_response() {
          lsp.resolve_pending(msg.get_id().unwrap(), msg_string);
        } else {
          //println!("{:?}", msg_string);
        }
      }
    });

    let params = InitializeParams {
      process_id: Some(std::process::id() as u32),
      workspace_folders: Some(workspace_folders),
      capabilities: capabilities,
      work_done_progress_params: lsp_types::WorkDoneProgressParams {
        work_done_token: Some(NumberOrString::String(self.working_token)),
      },
      ..Default::default()
    };

    //wait for listener to start
    sleep(std::time::Duration::from_millis(1));

    let init: LSPRequest<Initialize> = LSPRequest::new(Some(params.clone()))?;
    let _init_res = lsp.send_req(init).await?;

    let inited: LSPNotification<Initialized> = LSPNotification::new(None)?;
    lsp.send_not(inited)?;

    Ok(lsp)
  }
}

impl LSP {
  pub(crate) async fn send_req<T>(&self, req: LSPRequest<T>) -> Result<LSPResponse<T>, Error>
  where
    T: LSPRequestTrait,
  {
    let mut lsp = self.lsp_client.lock().unwrap();
    let id = req.get_id();

    let res = PendingRequest {
      id,
      condition: Arc::new((Mutex::new(false), Condvar::new())),
      response: None,
    };

    lsp.pending.push(res);
    lsp.send_req(req)?;

    let (lock, cvar) = &*lsp.pending.last().unwrap().condition.clone();
    let mut started = lock.lock().unwrap();
    drop(lsp);

    while !*started {
      started = cvar.wait(started).unwrap();
    }

    let lsp = self.lsp_client.lock().unwrap();
    let res = lsp.pending.iter().find(|p| p.id == id).unwrap();
    let response = res.response.clone().unwrap();
    let response = LSPResponse::new(response.clone())?;

    Ok(response)
  }

  pub(crate) fn send_not<T>(&self, not: LSPNotification<T>) -> Result<(), Error>
  where
    T: LSPNotificationTrait,
  {
    let mut lsp = self.lsp_client.lock().unwrap();
    lsp.send_not(not)
  }
}

fn read_msg(reader: &mut BufReader<std::process::ChildStdout>) -> Result<Option<String>, Error> {
  fn invalid_data(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
      io::Error::new(io::ErrorKind::InvalidData, error)
  }
  macro_rules! invalid_data {
      ($($tt:tt)*) => (invalid_data(format!($($tt)*)))
  }

  let mut size = None;
  let mut buf = String::new();
  loop {
    buf.clear();
    if reader.read_line(&mut buf)? == 0 {
      return Ok(None);
    }
    if !buf.ends_with("\r\n") {
      return Err(invalid_data!("Malformed header: {:?}", buf));
    }

    let buf = &buf[..buf.len() - 2];
    if buf.is_empty() {
      break;
    }
    let mut parts = buf.splitn(2, ": ");
    let key = parts.next().unwrap();
    let value = parts.next().ok_or_else(|| invalid_data!("Malformed header: {:?}", buf))?;

    if key.eq_ignore_ascii_case("Content-Length") {
      size = Some(value.parse::<usize>().map_err(invalid_data)?);
    }
  }
  let size = size.ok_or_else(|| invalid_data!("Missing Content-Length header"))?;
  let mut buf = buf.into_bytes();
  buf.resize(size, 0);
  reader.read_exact(&mut buf)?;
  let buf = String::from_utf8(buf).map_err(invalid_data)?;

  Ok(Some(buf))
}