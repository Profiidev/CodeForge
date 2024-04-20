use std::{
  io::{BufRead, BufReader, Error, Read, Write},
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
      let reader = BufReader::new(self.stdout);

      let mut buf = String::new();
      let mut open_brackets = 0;
      let mut is_in_string = false;
      let mut started = false;
      let mut last_escape = false;
      for byte in reader.bytes() {
        if let Err(e) = byte {
          println!("Error reading byte: {:?}", e);
          continue;
        }

        let byte = byte.unwrap() as char;
        buf.push(byte);

        match byte {
          '{' => {
            started = true;
            if !is_in_string {
              open_brackets += 1;
            }
          }
          '}' => {
            if !is_in_string {
              open_brackets -= 1;
            }
          }
          '"' => {
            if !last_escape {
              is_in_string = !is_in_string;
            }
          }
          _ => {}
        }
        last_escape = byte == '\\';

        if started && open_brackets == 0 {
          let body = buf.split("\r\n\r\n").collect::<Vec<&str>>()[1];
          let res = serde_json::from_str::<LSPMessage>(body);

          if let Ok(res) = res {
            if res.is_response() {
              let id = res.get_id().unwrap();
              let mut lsp = lsp_client.lock().unwrap();
              lsp.resolve_pending(id, body.to_string());
            }
          }

          buf.clear();
          started = false;
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
    let test = lsp.send_req(init).await?;
    println!("{:?}", test);

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
    println!("{:?}", response);
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

