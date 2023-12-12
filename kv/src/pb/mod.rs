pub mod abi;

use http::StatusCode;

use crate::{CommandRequest, Hget, Hgetall, Hset, KvError, Kvpair, Value};

use crate::pb::abi::CommandResponse;

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(crate::command_request::RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(crate::command_request::RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(crate::command_request::RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    pub fn new_hcontains(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(crate::command_request::RequestData::Hexist(crate::Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(crate::command_request::RequestData::Hdel(crate::Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            value: Some(crate::value::Value::String(value.into())),
        }
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(crate::value::Value::String(value)),
        }
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self {
            value: Some(crate::value::Value::Bool(value)),
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self {
            value: Some(crate::value::Value::Integer(value as i64)),
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value],
            ..Default::default()
        }
    }
}

/// 从 KvError 转换成 CommandResponse
impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            KvError::ConvertError(_, _) => todo!(),
            KvError::StorageError(_, _, _, _) => todo!(),
            KvError::EncodeError(_) => todo!(),
            KvError::DecodeError(_) => todo!(),
            KvError::Internal(_) => todo!(),
        }

        result
    }
}
/// 从 Vec<Kvpair> 转换成 CommandReponse
impl From<Vec<Kvpair>> for CommandResponse {
    fn from(value: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: value,
            ..Default::default()
        }
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(value: (String, Value)) -> Self {
        Kvpair::new(value.0, value.1)
    }
}
