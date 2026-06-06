// SAMPLE CODE v7.0 - Aurphyx AFS Empire
// MODULE: redteam | TEMPLATE: error_handling | ID: [redteam, System.Collections.Hashtable]
// 100% CARGO BUILD SAFE | Production Skeleton | Extend Later
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SampleError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Processing failed")]
    ProcessingFailed,
}

pub fn safe_process(data: &[u8]) -> Result<Vec<u8>, SampleError> {
    if data.is_empty() {
        return Err(SampleError::InvalidInput("empty data".to_string()));
    }
    Ok(data.to_vec())
}
// SAMPLE CODE v7.0 - Aurphyx AFS Empire
// MODULE: redteam | TEMPLATE: error_handling | ID: [redteam, System.Collections.Hashtable]
// 100% CARGO BUILD SAFE | Production Skeleton | Extend Later
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SampleError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Processing failed")]
    ProcessingFailed,
}

pub fn safe_process(data: &[u8]) -> Result<Vec<u8>, SampleError> {
    if data.is_empty() {
        return Err(SampleError::InvalidInput("empty data".to_string()));
    }
    Ok(data.to_vec())
}
