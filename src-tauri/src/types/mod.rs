mod activity;

pub use activity::ActivityError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    Activity(#[from] activity::ActivityError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[allow(dead_code)]
    #[error("This command is not implemented yet")]
    NotImplemented,
}

#[derive(Serialize)]
#[serde(tag = "error", content = "message")]
enum CommandErrorName {
    Activity(String),
    Sqlx(String),
    #[serde(rename = "Not implemented")]
    NotImplemented(String),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = self.to_string();
        let name = match self {
            CommandError::Activity(_) => CommandErrorName::Activity(message),
            CommandError::Sqlx(_) => CommandErrorName::Sqlx(message),
            CommandError::NotImplemented => CommandErrorName::NotImplemented(message),
        };

        name.serialize(serializer)
    }
}
