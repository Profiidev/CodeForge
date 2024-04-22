use core::fmt;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct LSPError {
  pub(crate) code: ErrorCode,
  pub(crate) message: String,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ErrorCode {
  ///Invalid JSON was received by the server.
  ///An error occurred on the server while parsing the JSON text.
  ParseError,
  ///The JSON sent is not a valid Request object.
  InvalidRequest,
  ///The method does not exist / is not available.
  MethodNotFound,
  ///Invalid method parameters.
  InvalidParams,
  ///Internal JSON-RPC error.
  InternalError,
  ///Reserved for implementation-defined server-errors.
  ServerError(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Version {
  V2,
}

struct VersionVisitor;

impl Serialize for Version {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    match *self {
      Version::V2 => ser.serialize_str("2.0"),
    }
  }
}

impl<'a> Deserialize<'a> for Version {
  fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
    des.deserialize_str(VersionVisitor)
  }
}

impl<'a> Visitor<'a> for VersionVisitor {
  type Value = Version;

  #[inline]
  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("Identifier must be a string and be of one the following: ['2.0']")
  }

  #[inline]
  fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
    match v {
      "2.0" => Ok(Version::V2),
      _ => Err(serde::de::Error::invalid_value(
        serde::de::Unexpected::Str(v),
        &self,
      )),
    }
  }
}

impl ErrorCode {
  pub const fn from_code(code: i64) -> Self {
    match code {
      -32700 => ErrorCode::ParseError,
      -32600 => ErrorCode::InvalidRequest,
      -32601 => ErrorCode::MethodNotFound,
      -32602 => ErrorCode::InvalidParams,
      -32603 => ErrorCode::InternalError,
      code => ErrorCode::ServerError(code),
    }
  }
}

impl<'a> Deserialize<'a> for ErrorCode {
  #[inline]
  fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<ErrorCode, D::Error> {
    let code: i64 = Deserialize::deserialize(deserializer)?;
    Ok(ErrorCode::from_code(code))
  }
}
