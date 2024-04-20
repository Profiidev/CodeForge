use std::io::Error;

use lsp_types::notification::Notification as LSPNotificationTrait;
use serde::{Deserialize, Serialize};

use super::utils::Version;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct LSPNotification<T>
where
  T: LSPNotificationTrait,
{
  jsonrpc: Version,
  method: String,
  params: Option<T::Params>,
}

impl<T: LSPNotificationTrait> LSPNotification<T> {
  pub(crate) fn new(body: Option<T::Params>) -> Result<LSPNotification<T>, Error> {
    Ok(LSPNotification {
      jsonrpc: Version::V2,
      method: T::METHOD.to_string(),
      params: body,
    })
  }

  pub(crate) fn as_bytes(self) -> Result<Vec<u8>, Error> {
    let not = serde_json::to_string(&self)?;
    let not = format!("Content-Length: {}\r\n\r\n{}", not.len(), not);
    Ok(not.as_bytes().to_vec())
  }
}
