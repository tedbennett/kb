#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
pub enum KbError {
    FailedToCreateFile,
    FailedToWriteFile,
    FailedToReadFile,
    DirDoesNotExist,
    FileDoesNotExist,
    Unknown(color_eyre::Report),
}

impl From<color_eyre::Report> for KbError {
    fn from(value: color_eyre::Report) -> Self {
        KbError::Unknown(value)
    }
}

impl fmt::Display for KbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KbError::FailedToCreateFile => write!(f, "Failed to create file."),
            KbError::FailedToWriteFile => write!(f, "Failed to write to file."),
            KbError::FailedToReadFile => write!(f, "Failed to read file."),
            KbError::DirDoesNotExist => write!(f, "Directory does not exist."),
            KbError::FileDoesNotExist => write!(f, "File does not exist."),
            KbError::Unknown(err) => write!(f, "{}", err),
        }
    }
}
