use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Failed to save todos: {0}")]
    SaveError(String),
    #[error("Failed to load todos: {0}")]
    LoadError(String),
    #[error("UI error: {0}")]
    UIError(#[from] iced::Error),
}

pub type Result<T> = std::result::Result<T, TodoError>; 