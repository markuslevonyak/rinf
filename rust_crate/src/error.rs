use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
  NoDartIsolate,
  CannotEncodeMessage,
  CannotDecodeMessage,
  NoBindings,
}

impl Error for AppError {}

impl Display for AppError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::NoDartIsolate => {
        write!(f, "Dart isolate for Rust signals was not created")
      }
      Self::CannotDecodeMessage => {
        write!(f, "Could not decode the message")
      }
      Self::CannotEncodeMessage => {
        write!(f, "Could not encode the message")
      }
      Self::NoBindings => {
        write!(f, "Rinf bindings are not ready")
      }
    }
  }
}
