use std::io::Error;

use lsp_types::request::Request as LSPRequestTrait;
use serde::Deserialize;

use super::utils::{LSPError, Version};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct LSPResponseBuilder<T>
where
  T: LSPRequestTrait,
{
  #[serde(rename = "jsonrpc")]
  _jsonrpc: Version,
  result: Option<T::Result>,
  error: Option<LSPError>,
  id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LSPResponse<T>
where
  T: LSPRequestTrait,
{
  result: Result<T::Result, LSPError>,
  id: Option<i32>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct LSPMessage {
  #[serde(rename = "jsonrpc")]
  _jsonrpc: Version,
  id: Option<i32>,
}

impl<T: LSPRequestTrait> LSPResponse<T> {
  pub(crate) fn new(body: String) -> Result<LSPResponse<T>, Error> {
    let res: LSPResponseBuilder<T> = serde_json::from_str(&body)?;

    Ok(LSPResponse {
      result: match res.result {
        Some(result) => Ok(result),
        None => Err(res.error.unwrap()),
      },
      id: res.id,
    })
  }
}

impl LSPMessage {
  pub(crate) fn is_response(&self) -> bool {
    self.id.is_some()
  }

  pub(crate) fn get_id(&self) -> Option<i32> {
    self.id
  }
}