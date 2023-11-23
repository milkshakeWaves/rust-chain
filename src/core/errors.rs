use std::fmt;

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
        AppendToHistoryError{}
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
        TransactionValidationError{}
    }
}