//! Errors within module [crate::jobs].

use thiserror::Error;

/// Error within module [super]
#[derive(Error, Debug)]
pub enum Error {
    /// Element already existing.
    #[error("Duplicate element {0}")]
    Duplicate(String),
}

/// Result within module [super]
pub type Result<T> = std::result::Result<T, Error>;
