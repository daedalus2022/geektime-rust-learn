use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not found for table:{0}, key:{1}")]
    NotFound(String, String),

    #[error("Command is invalid: `{0}`")]
    InvalidCommand(String),
}
