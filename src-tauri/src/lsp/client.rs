use std::{
  io::{BufRead, BufReader, Read, Write},
  os::windows::process::CommandExt,
  process::Command,
  sync::{Arc, Condvar, Mutex},
  thread::{self, sleep},
};

use anyhow::Error;
use lsp_types::{
  notification::{Initialized, Notification as LSPNotificationTrait},
  request::{Initialize, Request as LSPRequestTrait},
  ClientCapabilities, InitializeParams, NumberOrString, WorkspaceFolder,
};
use regex::Regex;

use super::{
  info::LSPInfo,
  notification::{LSPNotification, RawLSPNotification},
  request::{LSPRequest, PendingRequest},
  response::{LSPMessage, LSPResponse},
};

#[derive(Debug)]
pub(crate) struct LSPClient {
  stdin: std::process::ChildStdin,
  pending: Vec<PendingRequest>,
}

#[derive(Debug)]
pub(crate) struct LSPClientBuilder {
  path: Option<String>,
  args: Option<Vec<String>>,
  file_patterns: Option<Vec<String>>,
  workspace_folders: Option<Vec<WorkspaceFolder>>,
  capabilities: Option<ClientCapabilities>,
  not_handler: Option<fn(RawLSPNotification)>,
}

#[derive(Debug, Clone)]
pub(crate) struct LSPData {
  pub(self) lsp_client: Arc<Mutex<LSPClient>>,
  pub(super) file_patterns: Vec<Regex>,
  pub(crate) lsp_info: Option<LSPInfo>,
}

impl LSPClient {
  pub(crate) fn send_req<T>(&mut self, req: LSPRequest<T>) -> Result<(), Error>
  where
    T: LSPRequestTrait,
  {
    self.stdin.write_all(req.as_bytes()?.as_slice())?;
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
    self.stdin.write_all(not.as_bytes()?.as_slice())?;
    self.stdin.flush()?;
    Ok(())
  }
}

impl LSPClientBuilder {
  pub(crate) fn path(mut self, path: String) -> Self {
    self.path = Some(path);
    self
  }

  pub(crate) fn args(mut self, args: Vec<String>) -> Self {
    self.args = Some(args);
    self
  }

  pub(crate) fn file_patterns(mut self, file_patterns: Vec<String>) -> Self {
    self.file_patterns = Some(file_patterns);
    self
  }

  pub(crate) fn workspace_folders(mut self, workspace_folders: Vec<WorkspaceFolder>) -> Self {
    self.workspace_folders = Some(workspace_folders);
    self
  }

  pub(crate) fn capabilities(mut self, capabilities: ClientCapabilities) -> Self {
    self.capabilities = Some(capabilities);
    self
  }

  pub(crate) fn not_handler(mut self, not_handler: fn(RawLSPNotification)) -> Self {
    self.not_handler = Some(not_handler);
    self
  }

  pub(crate) async fn build(self) -> Result<LSPData, Error> {
    let path = self.path.ok_or(anyhow::anyhow!("No path"))?;
    let args = self.args.unwrap_or_default();
    let file_patterns = self.file_patterns.ok_or(anyhow::anyhow!("No file patterns"))?;
    let workspace_folders = self.workspace_folders.ok_or(anyhow::anyhow!("No workspace folders"))?;
    let capabilities = self.capabilities.ok_or(anyhow::anyhow!("No capabilities"))?;
    let not_handler = self.not_handler.unwrap_or(|_| ());

    let mut file_patterns_reg: Vec<Regex> = Vec::new();
    for pattern in file_patterns {
      let reg = Regex::new(&pattern)?;
      file_patterns_reg.push(reg);
    }

    let mut lsp = Command::new(path);

    #[cfg(target_os = "windows")]
    let lsp = lsp.creation_flags(0x08000000);

    let mut lsp = lsp
      .stdin(std::process::Stdio::piped())
      .stdout(std::process::Stdio::piped())
      .args(args)
      .spawn()?;

    let stdin = lsp
      .stdin
      .take()
      .ok_or(anyhow::anyhow!("failed to open stdin"))?;

    let stdout = lsp
      .stdout
      .take()
      .ok_or(anyhow::anyhow!("failed to open stdout"))?;

    let working_token = String::from("init_token");

    let lsp_client = LSPClient {
      stdin,
      pending: Vec::new(),
    };

    let mut lsp = LSPData {
      lsp_client: Arc::new(Mutex::new(lsp_client)),
      file_patterns: file_patterns_reg,
      lsp_info: None,
    };

    let lsp_client = lsp.lsp_client.clone();
    thread::spawn(move || {
      let mut reader = BufReader::new(stdout);

      while let Ok(Some(msg_string)) = read_msg(&mut reader) {
        let msg: LSPMessage = match serde_json::from_str(&msg_string) {
          Ok(msg) => msg,
          Err(_) => continue,
        };

        if msg.is_response() {
          let mut lsp = lsp_client.lock().unwrap();
          lsp.resolve_pending(msg.get_id().unwrap(), msg_string);
        } else {
          let not: RawLSPNotification = match serde_json::from_str(&msg_string) {
            Ok(not) => not,
            Err(_) => continue,
          };

          thread::spawn(move || not_handler(not));
        }
      }
    });

    let params = InitializeParams {
      process_id: Some(std::process::id()),
      workspace_folders: Some(workspace_folders),
      capabilities,
      work_done_progress_params: lsp_types::WorkDoneProgressParams {
        work_done_token: Some(NumberOrString::String(working_token)),
      },
      ..Default::default()
    };

    //wait for listener to start
    sleep(std::time::Duration::from_millis(1));

    let init: LSPRequest<Initialize> = LSPRequest::new(Some(params.clone()));
    let init_res = lsp.send_req(init).await?;

    let lsp_info = LSPInfo::new(init_res.ok_or(anyhow::anyhow!("No Init Response"))?)?;
    println!("{:?}", lsp_info);
    lsp.lsp_info = Some(lsp_info);

    let inited: LSPNotification<Initialized> = LSPNotification::new(None)?;
    lsp.send_not(inited)?;

    Ok(lsp)
  }
}

fn read_msg(reader: &mut BufReader<std::process::ChildStdout>) -> Result<Option<String>, Error> {
  let mut size = None;
  let mut buf = String::new();
  loop {
    buf.clear();
    if reader.read_line(&mut buf)? == 0 {
      return Ok(None);
    }
    if !buf.ends_with("\r\n") {
      return Err(anyhow::anyhow!("Malformed header: {:?}", buf));
    }

    let buf = &buf[..buf.len() - 2];
    if buf.is_empty() {
      break;
    }
    let mut parts = buf.splitn(2, ": ");
    let key = parts.next().unwrap();
    let value = parts
      .next()
      .ok_or_else(|| anyhow::anyhow!("Malformed header: {:?}", buf))?;

    if key.eq_ignore_ascii_case("Content-Length") {
      size = Some(value.parse::<usize>()?);
    }
  }
  let size = size.ok_or_else(|| anyhow::anyhow!("Missing Content-Length header"))?;
  let mut buf = buf.into_bytes();
  buf.resize(size, 0);
  reader.read_exact(&mut buf)?;
  let buf = String::from_utf8(buf)?;

  Ok(Some(buf))
}

impl LSPData {
  pub(crate) fn create() -> LSPClientBuilder {
    LSPClientBuilder {
      path: None,
      args: None,
      file_patterns: None,
      workspace_folders: None,
      capabilities: None,
      not_handler: None,
    }
  }

  pub(crate) async fn send_req<T>(
    &self,
    req: LSPRequest<T>,
  ) -> Result<Option<LSPResponse<T>>, Error>
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
    let response = res.response.clone().unwrap_or_default();
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
