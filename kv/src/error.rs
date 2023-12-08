use thiserror::Error;

#[warn(named_arguments_used_positionally)]
#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not found for table:{0}, key:{1}")]
    NotFound(String, String),

    #[error("Command parse command: `{0}`")]
    InvalidCommand(String),

    #[error("Cannot convert value {0} to {1}")]
    ConvertError(String, String),

    #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {3}")]
    StorageError(String, String, String, String),

    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),

    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),

    #[error("Internal error: {0}")]
    Internal(String),
}
