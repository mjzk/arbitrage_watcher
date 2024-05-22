use alloy::hex::FromHexError;
use alloy::primitives::ruint::ParseError;
use alloy::transports::TransportErrorKind;
use alloy::{primitives::utils::UnitsError, transports::RpcError};
use thiserror::Error;

pub type ChainXResult<T> = Result<T, ChainXError>;

#[derive(Error, Debug)]
pub enum ChainXError {
    #[error("Parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("Units error: {0}")]
    Units(#[from] UnitsError),
    #[error("Rpc error: {0}")]
    RpcError(#[from] RpcError<TransportErrorKind>),
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Parse error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    FromHexError(#[from] FromHexError),
    #[error("Parse error: {0}")]
    CsvError(#[from] csv::Error),
}
