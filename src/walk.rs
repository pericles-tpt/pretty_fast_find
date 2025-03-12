use regex::bytes::Regex;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::find::FoundFile;

const HIDDEN_RX_STR: &str = r".*\/\..*";

pub fn walk_match_until_limit(initial_dirs: &mut Vec<std::path::PathBuf>, limit: usize, match_rx: Regex, match_exact: Option<&OsStr>) -> std::io::Result<(Vec<PathBuf>, Vec<FoundFile>)> {
    let mut dir_q: Vec<PathBuf> = std::mem::take(initial_dirs);
    let mut matches: Vec<FoundFile>;
    
    // Actual limit should be min(limit, some.len())
    let mut fd_limit = limit;
    if limit < dir_q.len() {
        fd_limit = dir_q.len();
    }
    matches = Vec::with_capacity(fd_limit);
    
    let mut f_idx = 0;
    let mut d_idx = 0;
    let hidden_rx = Regex::new(&HIDDEN_RX_STR).unwrap();
    let is_match_exact = match_exact.is_some();
    while (f_idx + d_idx) < fd_limit && d_idx < dir_q.len() {
        let dir_name = dir_q[d_idx].file_name();
        let parent_path_os_str = dir_q[d_idx].to_path_buf().into_os_string();
        let parent_hidden = hidden_rx.is_match(parent_path_os_str.as_bytes());
        if (is_match_exact && dir_name == match_exact) || 
           (!is_match_exact && match_rx.is_match(dir_name.unwrap().as_bytes())) {
            // Add trailing '/' to dir paths to differentiate them
            let mut parent_path_string = parent_path_os_str.into_string().unwrap();
            parent_path_string.push('/');
            let ent = FoundFile {
                s_path: parent_path_string,
                is_file: false,
                is_symlink: false,
                is_hidden: parent_hidden,
            };
            matches.push(ent);
        }
        
        let dir_entries = std::fs::read_dir(&dir_q[d_idx])?;
        d_idx += 1;
        for ent in dir_entries {
            let Ok(val) = ent else { continue };
            let Ok(ft) = val.file_type() else { continue };
            f_idx += 1;
            
            if ft.is_file() || ft.is_symlink() {
                let file_name = val.file_name();
                if (is_match_exact && file_name.as_os_str() == match_exact.unwrap()) || 
                   (!is_match_exact && match_rx.is_match(file_name.as_bytes())) {
                    let file_path_string = val.path().into_os_string().into_string().unwrap();
                    let ent = FoundFile {
                        s_path: file_path_string,
                        is_file: ft.is_file(),
                        is_symlink: ft.is_symlink(),
                        is_hidden: parent_hidden || file_name.as_bytes().starts_with(&['.' as u8]),
                    };
                    matches.push(ent);
                }
                continue;
            }

            dir_q.push(val.path());
        }
    }

    return Ok((dir_q.drain(d_idx..).collect(), matches));
}