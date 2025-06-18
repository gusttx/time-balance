use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntryError {
    #[error("Duration cannot be negative")]
    NegativeDuration,

    #[error("Activity not found")]
    ActivityNotFound,

    #[error("Page cannot be zero or negative")]
    ZeroOrNegativePage,
}
