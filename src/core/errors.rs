use std::fmt;

use hex::FromHexError;

#[derive(Debug, Clone)]
pub struct AppendToHistoryError;

impl fmt::Display for AppendToHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot append block to history")
    }
}

impl From<FromHexError> for AppendToHistoryError {
    fn from(err: FromHexError) -> AppendToHistoryError {
        AppendToHistoryError{}
    }
}