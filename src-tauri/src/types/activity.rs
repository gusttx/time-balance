use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActivityError {
    #[error("Activity name cannot be empty")]
    EmptyName,
}