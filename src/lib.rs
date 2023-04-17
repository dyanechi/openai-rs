use easydev::builder::*;
use std::fmt::{Display, Formatter, self};
pub use serde::{Serialize, Deserialize};

pub mod prelude;

pub mod openai;
pub use openai::*;
pub mod api;
pub use api::*;
pub mod memory;
pub use memory::*;

use prelude::*;

mod request;

pub type Json = serde_json::Value;
pub type JsonErr<T> = JsonError<T>;
pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonError<T> {
    error: Option<T>
}

#[derive(Debug)]
pub enum Error {
	/// An Error returned by the API
	ApiError(String),
	/// An Error not related to the API
	RequestError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiError(msg) => write!(f, "API error: {}", msg),
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
