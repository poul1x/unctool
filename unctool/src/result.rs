use std::{fmt::Display};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidPathFormat,
    LocalPathNotFound,
    RemotePathNotFound,
    ReadProcMountsFailed,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPathFormat => write!(f, r"Provided path is not in \\windows-share\ or smb://linux-share/ format"),
            Self::LocalPathNotFound => write!(f, "Local mountpoint is not found for this share path"),
            Self::RemotePathNotFound => write!(f, "Remote share is not found for this path"),
            Self::ReadProcMountsFailed => write!(f, "Failed to read /proc/mounts"),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;