//! # [unctool-gui](https://github.com/poul1x/unctool)
//!
//! A CLI tool to seamlessly convert between Linux and Windows UNC paths.
//! It can convert local Linux path to Windows/Linux UNC and vice versa.
//!
//! # Usage
//!
//! Convert between Linux and Windows UNC:
//!

use argh::{FromArgValue, FromArgs};
use std::path::Path;
use std::process::exit;
use unctool;

mod app_submit;
mod app_results;

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
    #[argh(option, default = "1.0")]
    /// the UI scaling factor, value of 1.0 represents 100% scaling (default)
    ui_scale: f32,
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

fn abspath(p: &str) -> Option<String> {
    let expanded_path = shellexpand::full(p).ok()?;
    let canonical_path = std::fs::canonicalize(expanded_path.as_ref()).ok()?;
    canonical_path.into_os_string().into_string().ok()
}

fn handle_app_exit(result: iced::Result) -> ! {
    match result {
        Ok(_) => {
            exit(0);
        }
        Err(e) => {
            eprintln!("[Fatal] Unable to run app: {}", e.to_string());
            exit(1);
        }
    }
}

fn run_app_submit(unctool: CmdUncTool) -> ! {
    let init_context = app_submit::InitContext {
        ui_settings: app_submit::UISettings {
            scale_factor: unctool.ui_scale,
        },
    };

    handle_app_exit(app_submit::run(init_context));
}

fn format_error(path: String, err_msg: &str) -> String {
    format!("{}. Path: '{}'", err_msg, path)
}

fn run_app_results(unctool: CmdUncTool) -> ! {
    let ui_settings = app_results::UISettings {
        scale_factor: unctool.ui_scale,
    };

    match unctool.subcommand.unwrap() {
        CmdUncToolSub::Convert(cmd_convert) => {
            let path = cmd_convert.path;
            let path_type = cmd_convert.path_type;

            let init_context = match unctool::convert_unc(&path, path_type.into()) {
                Ok(val) => app_results::InitContext {
                    is_success: true,
                    text_value: val,
                    ui_settings: ui_settings,
                },
                Err(e) => app_results::InitContext {
                    is_success: false,
                    text_value: format_error(path, e.to_string().as_str()),
                    ui_settings: ui_settings,
                },
            };

            handle_app_exit(app_results::run(init_context));
        }
        CmdUncToolSub::LocalPath(cmd_local_path) => {
            let path = cmd_local_path.remote_path;
            let init_context = match unctool::local_path(&path) {
                Ok(val) => app_results::InitContext {
                    is_success: true,
                    text_value: val,
                    ui_settings: ui_settings,
                },
                Err(e) => app_results::InitContext {
                    is_success: false,
                    text_value: format_error(path, e.to_string().as_str()),
                    ui_settings: ui_settings,
                },
            };

            handle_app_exit(app_results::run(init_context));
        }
        CmdUncToolSub::RemotePath(cmd_remote_path) => {
            let path = cmd_remote_path.local_path;
            let path_type = cmd_remote_path.path_type;

            let init_context: app_results::InitContext = {
                if Path::new(&path).exists() {
                    if let Some(abs_path) = abspath(&path) {
                        match unctool::remote_path(&abs_path, path_type.into()) {
                            Ok(val) => app_results::InitContext {
                                is_success: true,
                                text_value: val,
                                ui_settings: ui_settings,
                            },
                            Err(e) => app_results::InitContext {
                                is_success: false,
                                text_value: format_error(path, e.to_string().as_str()),
                                ui_settings: ui_settings,
                            },
                        }
                    } else {
                        app_results::InitContext {
                            is_success: false,
                            text_value: format_error(path, "Path does not exist or access denied"),
                            ui_settings: ui_settings,
                        }
                    }
                } else {
                    app_results::InitContext {
                        is_success: false,
                        text_value: format_error(path, "Path does not exist or access denied"),
                        ui_settings: ui_settings,
                    }
                }
            };

            handle_app_exit(app_results::run(init_context));
        }
    }
}

fn main() -> ! {
    let unctool: CmdUncTool = argh::from_env();
    if unctool.subcommand.is_none() {
        run_app_submit(unctool);
    } else {
        run_app_results(unctool);
    }
}
