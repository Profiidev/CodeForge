use std::{io::Error, sync::{Arc, Condvar, Mutex}};

use lsp_types::request::Request as LSPRequestTrait;
use rand::random;
use serde::Serialize;

use super::utils::Version;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct LSPRequest<T>
where
  T: LSPRequestTrait,
{
  jsonrpc: Version,
  method: String,
  params: Option<T::Params>,
  id: i32,
}

#[derive(Debug, Clone)]
pub(crate) struct PendingRequest {
  pub id: i32,
  pub condition: Arc<(Mutex<bool>, Condvar)>,
  pub response: Option<String>,
}

impl<T: LSPRequestTrait> LSPRequest<T> {
  pub(crate) fn new(body: Option<T::Params>) -> LSPRequest<T> {
    let id = random::<i32>().abs();

    LSPRequest {
      jsonrpc: Version::V2,
      method: T::METHOD.to_string(),
      params: body,
      id,
    }
  }

  pub(crate) fn get_id(&self) -> i32 {
    self.id
  }

  pub(crate) fn as_bytes(self) -> Result<Vec<u8>, Error> {
    let req = serde_json::to_string(&self)?;
    let req = format!("Content-Length: {}\r\n\r\n{}", req.len(), req);
    Ok(req.as_bytes().to_vec())
  }
}
