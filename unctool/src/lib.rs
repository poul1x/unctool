//! # [unctool](https://github.com/poul1x/unctool)
//!
//! A library to convert between Linux and Windows UNC paths.
//! It can convert local Linux path (mount)
//! to Windows/Linux UNC and vice versa.

#![cfg_attr(test, allow(dead_code))]

mod mounts;
mod result;
pub use crate::result::Error;
pub use crate::result::Result;

const OS_SEP_WINDOWS: &str = r"\";
const OS_SEP_LINUX: &str = r"/";

const UNC_PREFIX_WINDOWS: &str = r"\\";
const UNC_PREFIX_LINUX: &str = r"smb://";

#[derive(Debug, PartialEq)]
pub enum PathType {
    Windows,
    Linux,
}

fn remote_path_to_intermediate(path: &str) -> Result<String> {
    if path.starts_with(UNC_PREFIX_WINDOWS) {
        Ok(path.replace(OS_SEP_WINDOWS, OS_SEP_LINUX))
    } else if path.starts_with(UNC_PREFIX_LINUX) {
        Ok(path[UNC_PREFIX_LINUX.len() - 2..].to_owned())
    } else {
        Err(Error::InvalidPathFormat)
    }
}

fn intermediate_path_to_remote(path: &str, path_type: PathType) -> String {
    match path_type {
        PathType::Windows => path.replace(OS_SEP_LINUX, OS_SEP_WINDOWS),
        PathType::Linux => UNC_PREFIX_LINUX[..UNC_PREFIX_LINUX.len() - 2].to_owned() + path,
    }
}

/// Convert between Windows UNC and Linux UNC paths
///
/// **Parameters**:
/// - `path` - source path in UNC format
/// - `path_type` - destination UNC path type
///
/// **Returns**: converted UNC path wrapped with [`crate::Result`]
///
/// # Example
///
/// ```
/// let invalid_path = r"\\mynas\some\path";
/// let res = unctool::convert_unc(invalid_path, unctool::PathType::Linux).unwrap();
/// assert_eq!(res, "smb://mynas/some/path".to_string());
/// ```
///
/// # Errors
///
/// Function will return [`Error::InvalidPathFormat`]`
/// if provided path is not in UNC format
///
/// ```
/// let invalid_path = "invalid-path";
/// let res = unctool::convert_unc(invalid_path, unctool::PathType::Linux);
/// assert_eq!(res, Err(unctool::Error::InvalidPathFormat));
/// ```
///
pub fn convert_unc(path: &str, path_type: PathType) -> Result<String> {
    let im_path = remote_path_to_intermediate(path)?;
    Ok(intermediate_path_to_remote(&im_path, path_type))
}

/// Convert Windows/Linux UNC path to local mounted filesystem path
///
/// **Parameters**:
/// - `path` - path in UNC format
///
/// **Returns**: local filesystem path wrapped with [`crate::Result`]
///
/// # Example
///
/// ```no_run
/// let res = unctool::local_path(r"\\mynas\some\path").unwrap();
/// assert_eq!(res, r"/mnt/mynas/some/path");
/// ```
///
/// # Errors
///
/// Function will return [`Error::InvalidPathFormat`]
/// if provided path is not in UNC format
///
/// ```
/// let invalid_path = r"invalid-path";
/// let err = unctool::local_path(invalid_path).unwrap_err();
/// assert_eq!(err, unctool::Error::InvalidPathFormat);
/// ```
///
/// Function will return [`Error::LocalPathNotFound`]
/// if local mountpoint is not found for provided UNC path
///
/// ```
/// let invalid_path2 = r"smb://aaa";
/// let err = unctool::local_path(invalid_path2).unwrap_err();
/// assert_eq!(err, unctool::Error::LocalPathNotFound);
/// ```
///
pub fn local_path(path: &str) -> Result<String> {
    let im_path = remote_path_to_intermediate(path)?;
    mounts::cifs_local_path(&im_path)
}

/// Convert local mounted filesystem path to Windows/Linux UNC path
///
/// **Parameters**:
/// - `path` - local filesystem path
/// - `path_type` - destination UNC path type
///
/// **Returns**: UNC path wrapped with [`crate::Result`]
///
/// # Example
///
/// ```no_run
/// let path = "/mnt/mynas/some/path";
/// let res = unctool::remote_path(path, unctool::PathType::Windows).unwrap();
/// assert_eq!(res, r"\\mynas\some\path");
/// ```
///
/// # Errors
///
/// Function will return [`Error::RemotePathNotFound`]
/// if remote share is not found for this path
///
/// ```
/// let path = r"/mnt/no-such-share";
/// let res = unctool::remote_path(path, unctool::PathType::Windows);
/// assert_eq!(res, Err(unctool::Error::RemotePathNotFound));
/// ```
///
pub fn remote_path(path: &str, path_type: PathType) -> Result<String> {
    let im_path = mounts::cifs_remote_path(path)?;
    Ok(intermediate_path_to_remote(&im_path, path_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    const REMOTE_PATH_WINDOWS: &str = r"\\mynas\some\path";
    const REMOTE_PATH_LINUX: &str = r"smb://mynas/some/path";
    const LOCAL_PATH_LINUX: &str = r"/mnt/mynas/some/path";

    #[test]
    fn test_convert_unc() {
        let res = convert_unc(REMOTE_PATH_WINDOWS, PathType::Linux).unwrap();
        assert_eq!(res, REMOTE_PATH_LINUX);

        let res = convert_unc(REMOTE_PATH_WINDOWS, PathType::Windows).unwrap();
        assert_eq!(res, REMOTE_PATH_WINDOWS);

        let res = convert_unc(REMOTE_PATH_LINUX, PathType::Windows).unwrap();
        assert_eq!(res, REMOTE_PATH_WINDOWS);

        let res = convert_unc(REMOTE_PATH_LINUX, PathType::Linux).unwrap();
        assert_eq!(res, REMOTE_PATH_LINUX);

        let invalid_path = "invalid-path";
        let err = convert_unc(invalid_path, PathType::Linux).unwrap_err();
        assert_eq!(err, Error::InvalidPathFormat);
    }

    #[test]
    fn test_local_path() {
        let res = local_path(REMOTE_PATH_WINDOWS).unwrap();
        assert_eq!(res, LOCAL_PATH_LINUX);

        let res = local_path(REMOTE_PATH_LINUX).unwrap();
        assert_eq!(res, LOCAL_PATH_LINUX);
    }

    #[test]
    fn test_local_path_invalid_data() {
        let invalid_path = r"no-such-path";
        let invalid_path2 = r"smb://";
        let invalid_path3 = r"smb://aaa/bbb/ccc";
        let invalid_path4 = r"\\";
        let invalid_path5 = r"\\aaa\bbb\ccc";

        let err = local_path(invalid_path).unwrap_err();
        assert_eq!(err, Error::InvalidPathFormat);

        let err = local_path(invalid_path2).unwrap_err();
        assert_eq!(err, Error::LocalPathNotFound);

        let err = local_path(invalid_path3).unwrap_err();
        assert_eq!(err, Error::LocalPathNotFound);

        let err = local_path(invalid_path4).unwrap_err();
        assert_eq!(err, Error::LocalPathNotFound);

        let err = local_path(invalid_path5).unwrap_err();
        assert_eq!(err, Error::LocalPathNotFound);
    }

    #[test]
    fn test_remote_path() {
        let res = remote_path(LOCAL_PATH_LINUX, PathType::Windows).unwrap();
        assert_eq!(res, REMOTE_PATH_WINDOWS);

        let res = remote_path(LOCAL_PATH_LINUX, PathType::Linux).unwrap();
        assert_eq!(res, REMOTE_PATH_LINUX);

        let invalid_path = r"no-such-path";
        let err = remote_path(invalid_path, PathType::Windows).unwrap_err();
        assert_eq!(err, Error::RemotePathNotFound);
    }
}
