use std::io::Error;

use lsp_types::notification::Notification as LSPNotificationTrait;
use serde::{Deserialize, Deserializer, Serialize};

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

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawLSPNotification {
  jsonrpc: Version,
  pub method: String,
  params: String,
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

impl RawLSPNotification {
  pub(crate) fn parse<T: LSPNotificationTrait>(&self) -> Result<LSPNotification<T>, Error> {
    let params = serde_json::from_str(&self.params)?;
    Ok(LSPNotification {
      jsonrpc: self.jsonrpc,
      method: self.method.clone(),
      params: Some(params),
    })
  }
}

impl<'de> Deserialize<'de> for RawLSPNotification {
  fn deserialize<D>(deserializer: D) -> Result<RawLSPNotification, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    let jsonrpc = Version::V2;
    let method = value["method"].as_str().unwrap_or_default().to_string();
    let params = value["params"].to_string();

    Ok(RawLSPNotification {
      jsonrpc,
      method,
      params,
    })
  }
    
}