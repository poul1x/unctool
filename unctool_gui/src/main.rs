//! # [unctool-cli](https://github.com/poul1x/unctool)
//!
//! A CLI tool to seamlessly convert between Linux and Windows UNC paths.
//! It can convert local Linux path to Windows/Linux UNC and vice versa.
//!
//! # Usage
//!
//! Convert between Linux and Windows UNC:
//!
//! ```bash
//! unctool convert 'smb://mynas.local/some/path' -t windows
//! # \\mynas.local\some\path
//!
//! unctool convert '\\mynas.local\some\path' -t linux
//! # smb://mynas.local/some/path
//! ```
//!
//! Convert to remote UNC:
//!
//! ```bash
//! unctool remote-path /mnt/mynas.local/some/path -t windows
//! # \\mynas.local\some\path
//!
//! unctool remote-path /mnt/mynas.local/some/path -t linux
//! # smb://mynas.local/some/path
//! ```
//!
//! Convert from remote UNC:
//!
//! ```bash
//! unctool local-path '\\mynas.local\some\path'
//! # /mnt/mynas.local/some/path
//!
//! unctool local-path 'smb://mynas.local/some/path'
//! # /mnt/mynas.local/some/path
//! ```

use argh::{FromArgValue, FromArgs};
use std::path::Path;
use std::process::exit;

use unctool;
mod gui_pre;
mod gui_post;

#[derive(Debug, PartialEq)]
pub enum PathType {
    Windows,
    Linux,
}

impl From<PathType> for unctool::PathType {
    fn from(value: PathType) -> Self {
        match value {
            PathType::Windows => unctool::PathType::Windows,
            PathType::Linux => unctool::PathType::Linux,
        }
    }
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

#[derive(FromArgs, PartialEq, Debug)]
#[argh(help_triggers("-h", "--help", "help"))]
/// UNC Tool - Seamlessly convert between Linux and Windows UNC paths.
/// Convert local Linux path to Windows/Linux UNC and vice versa.
struct CmdUncTool {
    #[argh(subcommand)]
    subcommand: Option<CmdUncToolSub>,
    #[argh(option, default="1.0")]
	/// aaa !!!!!!!!
    ui_scale: u32,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CmdUncToolSub {
    LocalPath(CmdLocalPath),
    RemotePath(CmdRemotePath),
    Convert(CmdConvert),
    Version(CmdVersion),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Show current version and exit
#[argh(subcommand, name = "version")]
struct CmdVersion {}

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

fn print_error(path: String, err_msg: String) {
    eprintln!("[Error] Failed to process '{}': {}", path, err_msg);
}

fn abspath(p: &str) -> Option<String> {
    let expanded_path = shellexpand::full(p).ok()?;
    let canonical_path = std::fs::canonicalize(expanded_path.as_ref()).ok()?;
    canonical_path.into_os_string().into_string().ok()
}

fn main() {
    let unctool: CmdUncTool = argh::from_env();
	gui_post::show();
    // match unctool.subcommand {
    //     CmdUncToolSub::Version(_) => {
    //         println!("unctool-cli {}", env!("CARGO_PKG_VERSION"));
    //         println!("unctool {}", unctool::version());
    //         exit(0);
    //     }
    //     CmdUncToolSub::Convert(cmd_convert) => {
    //         let path = cmd_convert.path;
    //         let path_type = cmd_convert.path_type;
    //         match unctool::convert_unc(&path, path_type.into()) {
    //             Ok(s) => {
    //                 println!("{}", s);
    //                 exit(0);
    //             }
    //             Err(e) => {
    //                 print_error(path, e.to_string());
    //                 exit(1);
    //             }
    //         }
    //     }
    //     CmdUncToolSub::LocalPath(cmd_local_path) => {
    //         let path = cmd_local_path.remote_path;
    //         match unctool::local_path(&path) {
    //             Ok(s) => {
    //                 println!("{}", s);
    //                 exit(0);
    //             }
    //             Err(e) => {
    //                 print_error(path, e.to_string());
    //                 exit(1);
    //             }
    //         }
    //     }
    //     CmdUncToolSub::RemotePath(cmd_remote_path) => {
    //         let path = cmd_remote_path.local_path;
    //         let path_type = cmd_remote_path.path_type;

    //         if !Path::new(&path).exists() {
    //             print_error(path, "Path does not exist or access denied".into());
    //             exit(1);
    //         }

    //         let abs_path = match abspath(&path) {
    //             Some(res) => res,
    //             None => {
    //                 print_error(path, "Failed to get an absolute path".into());
    //                 exit(1);
    //             }
    //         };

    //         match unctool::remote_path(&abs_path, path_type.into()) {
    //             Ok(s) => {
    //                 println!("{}", s);
    //                 exit(0);
    //             }
    //             Err(e) => {
    //                 print_error(path, e.to_string());
    //                 exit(1);
    //             }
    //         }
    //     }
    // }
}
