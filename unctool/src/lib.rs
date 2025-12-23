mod mounts;
mod result;
use crate::result::Error;
use crate::result::Result;

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

pub fn convert_unc(path: &str, path_type: PathType) -> Result<String> {
    let im_path = remote_path_to_intermediate(path)?;
    Ok(intermediate_path_to_remote(&im_path, path_type))
}

pub fn local_path(path: &str) -> Result<String> {
    let im_path = remote_path_to_intermediate(path)?;
    mounts::cifs_local_path(&im_path)
}

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
