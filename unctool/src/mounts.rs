use crate::{Error, Result};
use std::fs;

const CIFS: &str = r"cifs";
const MOUNTS_FILE: &str = r"/proc/mounts";

#[derive(Debug)]
struct CIFSEntry {
    local_path: String,
    remote_path: String,
}

fn decode_octal_symbols(encoded_str: &str) -> String {
    encoded_str
        .replace("\\040", " ")
        .replace("\\011", "\t")
        .replace("\\012", "\n")
        .replace("\\\\", "\\")
}

fn read_proc_mounts() -> Result<String> {
    fs::read_to_string(MOUNTS_FILE).map_err(|_| Error::ReadProcMountsFailed)
}

#[cfg(test)]
fn get_cifs_entries() -> Result<Vec<CIFSEntry>> {
    Ok(vec![CIFSEntry {
        local_path: "/mnt/mynas".into(),
        remote_path: "//mynas".into(),
    }])
}

#[cfg(not(test))]
fn get_cifs_entries() -> Result<Vec<CIFSEntry>> {
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

pub fn cifs_remote_path(path: &str) -> Result<String> {
    let entries = get_cifs_entries()?;
    match entries.iter().find(|e| path.starts_with(&e.local_path)) {
        Some(e) => Ok(path.replacen(&e.local_path, &e.remote_path, 1)),
        None => Err(Error::RemotePathNotFound),
    }
}

pub fn cifs_local_path(path: &str) -> Result<String> {
    let entries = get_cifs_entries()?;
    match entries.iter().find(|e| path.starts_with(&e.remote_path)) {
        Some(e) => Ok(path.replacen(&e.remote_path, &e.local_path, 1)),
        None => Err(Error::LocalPathNotFound),
    }
}
