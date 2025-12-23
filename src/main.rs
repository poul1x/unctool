use argh::{FromArgValue, FromArgs};
use std::path::Path;
use std::{fs, process::exit};
pub type Result<T> = core::result::Result<T, String>;

const CIFS: &str = r"cifs";
const MOUNTS_FILE: &str = r"/proc/mounts";

const OS_SEP_WINDOWS: &str = r"\";
const OS_SEP_LINUX: &str = r"/";

const UNC_PREFIX_WINDOWS: &str = r"\\";
const UNC_PREFIX_LINUX: &str = r"smb://";

#[derive(Debug)]
struct CIFSEntry {
    local_path: String,
    remote_path: String,
}

#[derive(Debug, PartialEq)]
enum PathType {
    Windows,
    Linux,
}

impl FromArgValue for PathType {
    fn from_arg_value(value: &str) -> std::result::Result<Self, String> {
        match value.to_lowercase().as_str() {
            "windows" => Ok(PathType::Windows),
            "linux" => Ok(PathType::Linux),
            _ => Err("Path type must be 'windows' or 'linux'".into()),
        }
    }
}

fn decode_octal_symbols(encoded_str: &str) -> String {
    encoded_str
        .replace("\\040", " ")
        .replace("\\011", "\t")
        .replace("\\012", "\n")
        .replace("\\\\", "\\")
}

fn read_proc_mounts() -> Result<String> {
    fs::read_to_string(MOUNTS_FILE).map_err(|_| format!("Failed to read '{MOUNTS_FILE}'"))
}

fn cifs_read_entries() -> Result<Vec<CIFSEntry>> {
    let mut result: Vec<CIFSEntry> = Vec::new();
    for line in read_proc_mounts()?.split("\n") {
        let items: Vec<&str> = line.split(" ").collect();
        if items.len() < 3 || items[2] != CIFS {
            continue;
        }

        let entry = CIFSEntry {
            local_path: decode_octal_symbols(items[1].into()),
            remote_path: decode_octal_symbols(items[0].into()),
        };

        result.push(entry);
    }

    Ok(result)
}

fn cifs_remote_path(path: &str, entries: &Vec<CIFSEntry>) -> Result<String> {
    match entries.iter().find(|e| path.starts_with(&e.local_path)) {
        Some(e) => Ok(path.replacen(&e.local_path, &e.remote_path, 1)),
        None => Err(format!("Remote path not found for '{path}'")),
    }
}

fn cifs_local_path(path: &str, entries: &Vec<CIFSEntry>) -> Result<String> {
    match entries.iter().find(|e| path.starts_with(&e.remote_path)) {
        Some(e) => Ok(path.replacen(&e.remote_path, &e.local_path, 1)),
        None => Err(format!("Local path not found for '{path}'")),
    }
}

fn unc_os_remote_to_intermediate(path: &str) -> Result<String> {
    if path.starts_with(UNC_PREFIX_WINDOWS) {
        Ok(path.replace(OS_SEP_WINDOWS, OS_SEP_LINUX))
    } else if path.starts_with(UNC_PREFIX_LINUX) {
        Ok(path[UNC_PREFIX_LINUX.len() - 2..].to_owned())
    } else {
        Err(r"Provided path is not in \\windows-share\ or smb://linux-share/ format".into())
    }
}

fn unc_intermediate_to_os_remote(path: &str, path_type: PathType) -> String {
    match path_type {
        PathType::Windows => path.replace(OS_SEP_LINUX, OS_SEP_WINDOWS),
        PathType::Linux => UNC_PREFIX_LINUX[..UNC_PREFIX_LINUX.len() - 2].to_owned() + path,
    }
}

fn unc_convert_os_remote(path: &str, path_type: PathType) -> Result<String> {
    let im_path = unc_os_remote_to_intermediate(path)?;
    Ok(unc_intermediate_to_os_remote(&im_path, path_type))
}

fn unc_os_remote_to_local(path: &str) -> Result<String> {
    let entries = cifs_read_entries()?;
    let im_path = unc_os_remote_to_intermediate(path)?;
    cifs_local_path(&im_path, &entries)
}

fn unc_local_to_os_remote(path: &str, path_type: PathType) -> Result<String> {
    let entries = cifs_read_entries()?;
    let im_path = cifs_remote_path(path, &entries)?;
    Ok(unc_intermediate_to_os_remote(&im_path, path_type))
}

fn abspath(p: &str) -> Option<String> {
    let expanded_path = shellexpand::full(p).ok()?;
    let canonical_path = std::fs::canonicalize(expanded_path.as_ref()).ok()?;
    canonical_path.into_os_string().into_string().ok()
}

#[derive(FromArgs, PartialEq, Debug)]
/// UNC Tool - Seamlessly convert between Linux and Windows UNC paths.
/// Convert local Linux path to Windows/Linux UNC and vice versa.
struct CmdUncTool {
    #[argh(subcommand)]
    subcommand: CmdUncToolSub,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CmdUncToolSub {
    LocalPath(CmdLocalPath),
    RemotePath(CmdRemotePath),
    Convert(CmdConvert),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Convert remote Windows/Linux UNC path to local Linux filesystem path
#[argh(subcommand, name = "local-path")]
struct CmdLocalPath {
    #[argh(positional)]
    /// remote UNC path in \\windows-share\path or smb://linux-share/path format
    remote_path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Convert local Linux filesystem path to remote Windows/Linux UNC path
#[argh(subcommand, name = "remote-path")]
struct CmdRemotePath {
    #[argh(positional)]
    /// local Linux filesystem path
    local_path: String,

    #[argh(option, short = 't')]
    /// destination UNC path type: windows or linux
    path_type: PathType,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Convert src UNC path to dst UNC path
#[argh(subcommand, name = "convert")]
struct CmdConvert {
    #[argh(positional)]
    /// remote UNC path in \\windows-share\path or smb://linux-share/path format
    path: String,

    #[argh(option, short = 't')]
    /// destination UNC path type: windows or linux
    path_type: PathType,
}

fn main() {
    let unctool: CmdUncTool = argh::from_env();
    match unctool.subcommand {
        CmdUncToolSub::Convert(cmd_convert) => {
            let path = cmd_convert.path;
            let path_type = cmd_convert.path_type;
            match unc_convert_os_remote(&path, path_type) {
                Ok(s) => {
                    println!("{}", s);
                    exit(0);
                }
                Err(e) => {
                    eprintln!("[Error] {}", e.to_string());
                    exit(1);
                }
            }
        }
        CmdUncToolSub::LocalPath(cmd_local_path) => {
            let path = cmd_local_path.remote_path;
            match unc_os_remote_to_local(&path) {
                Ok(s) => {
                    println!("{}", s);
                    exit(0);
                }
                Err(e) => {
                    eprintln!("[Error] {}", e.to_string());
                    exit(1);
                }
            }
        }
        CmdUncToolSub::RemotePath(cmd_remote_path) => {
            let path = cmd_remote_path.local_path;
            let path_type = cmd_remote_path.path_type;

            if !Path::new(&path).exists() {
                eprintln!("[Error] Path does not exist or access denied: '{path}'");
                exit(1);
            }

            let abs_path = match abspath(&path) {
                Some(res) => res,
                None => {
                    eprintln!("[Error] Failed to get an absolute path for '{path}'");
                    exit(1);
                }
            };

            match unc_local_to_os_remote(&abs_path, path_type) {
                Ok(s) => {
                    println!("{}", s);
                    exit(0);
                }
                Err(e) => {
                    eprintln!("[Error] {}", e.to_string());
                    exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_PATH_FORMAT: &str = r"\\windows-share\ or smb://linux-share/";
    const LOCAL_PATH_NOT_FOUND: &str = r"Local path not found";
    const REMOTE_PATH_NOT_FOUND: &str = r"Remote path not found";

    const REMOTE_PATH_WINDOWS: &str = r"\\mynas\some\path";
    const REMOTE_PATH_LINUX: &str = r"smb://mynas/some/path";
    const LOCAL_PATH_LINUX: &str = r"/mnt/mynas/some/path";
    const CIFS_ENTRY_LOCAL: &str = "/mnt/mynas";
    const CIFS_ENTRY_REMOTE: &str = "//mynas";

    fn _mock_cifs_entries() -> Vec<CIFSEntry> {
        // Create 5 mounts, where the 3rd is used in tests

        let mut cifs_entries: Vec<CIFSEntry> = Vec::new();
        for i in 0..2 {
            let entry = CIFSEntry {
                local_path: format!("/mnt/mynas{i}"),
                remote_path: format!("//mynas{i}"),
            };

            cifs_entries.push(entry);
        }

        let entry = CIFSEntry {
            local_path: CIFS_ENTRY_LOCAL.into(),
            remote_path: CIFS_ENTRY_REMOTE.into(),
        };

        cifs_entries.push(entry);

        for i in 3..5 {
            let entry = CIFSEntry {
                local_path: format!("/mnt/mynas{i}"),
                remote_path: format!("//mynas{i}"),
            };

            cifs_entries.push(entry);
        }

        cifs_entries
    }

    fn _unc_os_remote_to_local(path: &str, entries: &Vec<CIFSEntry>) -> Result<String> {
        let im_path = unc_os_remote_to_intermediate(path)?;
        cifs_local_path(&im_path, &entries)
    }

    fn _unc_local_to_os_remote(
        path: &str,
        path_type: PathType,
        entries: &Vec<CIFSEntry>,
    ) -> Result<String> {
        let im_path = cifs_remote_path(path, &entries)?;
        Ok(unc_intermediate_to_os_remote(&im_path, path_type))
    }

    #[test]
    fn test_unc_convert_remote() {
        let res = unc_convert_os_remote(REMOTE_PATH_WINDOWS, PathType::Linux).unwrap();
        assert_eq!(res, REMOTE_PATH_LINUX);

        let res = unc_convert_os_remote(REMOTE_PATH_WINDOWS, PathType::Windows).unwrap();
        assert_eq!(res, REMOTE_PATH_WINDOWS);

        let res = unc_convert_os_remote(REMOTE_PATH_LINUX, PathType::Windows).unwrap();
        assert_eq!(res, REMOTE_PATH_WINDOWS);

        let res = unc_convert_os_remote(REMOTE_PATH_LINUX, PathType::Linux).unwrap();
        assert_eq!(res, REMOTE_PATH_LINUX);

        let invalid_path = "invalid-path";
        let err = unc_convert_os_remote(invalid_path, PathType::Linux).unwrap_err();
        assert!(err.to_string().contains(INVALID_PATH_FORMAT));
    }

    #[test]
    fn test_unc_convert_remote_to_local() {
        let entries = _mock_cifs_entries();
        let res = _unc_os_remote_to_local(REMOTE_PATH_WINDOWS, &entries).unwrap();
        assert_eq!(res, LOCAL_PATH_LINUX);

        let res = _unc_os_remote_to_local(REMOTE_PATH_LINUX, &entries).unwrap();
        assert_eq!(res, LOCAL_PATH_LINUX);
    }

    #[test]
    fn test_unc_convert_remote_to_local_invalid_data() {
        let entries = _mock_cifs_entries();
        let invalid_path = r"no-such-path";
        let invalid_path2 = r"smb://";
        let invalid_path3 = r"smb://aaa/bbb/ccc";
        let invalid_path4 = r"\\";
        let invalid_path5 = r"\\aaa\bbb\ccc";

        let err = _unc_os_remote_to_local(invalid_path, &entries).unwrap_err();
        assert!(err.to_string().contains(INVALID_PATH_FORMAT));

        let err = _unc_os_remote_to_local(invalid_path2, &entries).unwrap_err();
        assert!(err.to_string().contains(LOCAL_PATH_NOT_FOUND));

        let err = _unc_os_remote_to_local(invalid_path3, &entries).unwrap_err();
        assert!(err.to_string().contains(LOCAL_PATH_NOT_FOUND));

        let err = _unc_os_remote_to_local(invalid_path4, &entries).unwrap_err();
        assert!(err.to_string().contains(LOCAL_PATH_NOT_FOUND));

        let err = _unc_os_remote_to_local(invalid_path5, &entries).unwrap_err();
        assert!(err.to_string().contains(LOCAL_PATH_NOT_FOUND));
    }

    #[test]
    fn test_unc_convert_local_to_remote() {
        let entries = _mock_cifs_entries();
        let res = _unc_local_to_os_remote(LOCAL_PATH_LINUX, PathType::Windows, &entries).unwrap();
        assert_eq!(res, REMOTE_PATH_WINDOWS);

        let res = _unc_local_to_os_remote(LOCAL_PATH_LINUX, PathType::Linux, &entries).unwrap();
        assert_eq!(res, REMOTE_PATH_LINUX);

        let invalid_path = r"no-such-path";
        let err = _unc_local_to_os_remote(invalid_path, PathType::Windows, &entries).unwrap_err();
        assert!(err.to_string().contains(REMOTE_PATH_NOT_FOUND));
    }
}
