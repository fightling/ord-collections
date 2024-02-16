//! Errors within module [crate::jobs].

#[cfg(feature = "thiserror")]
use thiserror::Error;

/// Error within module [super]
#[derive(Debug)]
#[cfg_attr(feature = "thiserror", derive(Error))]
pub enum Error {
    /// Element already existing.
    #[cfg_attr(feature = "thiserror", error("Duplicate element {0}"))]
    Duplicate(String),
}

/// Result within module [super]
pub type Result<T> = std::result::Result<T, Error>;
