use std::{fmt, io::Empty};

use hex::FromHexError;

use super::Transaction;

#[derive(Debug, Clone)]
pub struct AppendToHistoryError;

impl fmt::Display for AppendToHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot append block to history")
    }
}

impl From<FromHexError> for AppendToHistoryError {
    fn from(err: FromHexError) -> AppendToHistoryError {
        AppendToHistoryError {}
    }
}

#[derive(Debug, Clone)]
pub struct TransactionValidationError;

impl fmt::Display for TransactionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction failed to validate")
    }
}

impl From<FromHexError> for TransactionValidationError {
    fn from(err: FromHexError) -> TransactionValidationError {
        TransactionValidationError {}
    }
}

#[derive(Debug, Clone)]
pub struct EmptySignatureError {
    msg: String,
}

impl EmptySignatureError {
    pub fn new(error_msg: String) -> EmptySignatureError {
        EmptySignatureError { msg: error_msg }
    }
}

impl fmt::Display for EmptySignatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<secp256k1::Error> for EmptySignatureError {
    fn from(err: secp256k1::Error) -> EmptySignatureError {
        EmptySignatureError {
            msg: err.to_string(),
        }
    }
}
