extern crate queues;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use regex::bytes::Regex;

use crate::find::FoundFile;

pub fn walk_search_until_limit(target: &String, some: Vec<std::path::PathBuf>, other_entries: &mut Vec<FoundFile>, thread_readdir_limit: usize) -> std::io::Result<Vec<PathBuf>> {
    let mut readdir_limit = thread_readdir_limit;
    if readdir_limit < some.len() {
        readdir_limit = some.len();
    }
    
    let mut dir_q: Vec<PathBuf> = some;
    
    let mut dIdx = 0;
    let mut fIdx = 0;
    let Ok(rx) = Regex::new(target) else {return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("failed to make target rx")));};
    let hidden_rx = Regex::new(r".*\/\..*").unwrap();
    while (fIdx + dIdx) < readdir_limit && dIdx < dir_q.len() {
        let fnb: &[u8] = dir_q[dIdx].file_name().unwrap().as_bytes();
        let parent_path_os_str = dir_q[dIdx].to_path_buf().into_os_string();
        let parent_hidden = hidden_rx.is_match(parent_path_os_str.as_bytes());
        if rx.is_match(fnb) {
            let ent = FoundFile {
                p: parent_path_os_str.into_string().unwrap(),
                is_sym: false,
                is_hidden: parent_hidden,
            };
            other_entries.push(ent);
        }
        
        let rd = std::fs::read_dir(&dir_q[dIdx]);
        if rd.is_err() {
            // TODO: Handle error
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", rd.err())));
        }

        dIdx += 1;
        let Ok(entries) = rd else { continue; };
        for ent in entries {
            let Ok(val) = ent else { continue };
            let Ok(ft) = val.file_type() else { continue };
            fIdx += 1;
            
            if !ft.is_dir() {
                let file_name = val.file_name();
                let fnb = file_name.as_bytes();
                if rx.is_match(fnb) {
                    let ent = FoundFile {
                        p: val.path().into_os_string().into_string().unwrap(),
                        is_sym: ft.is_symlink(),
                        is_hidden: parent_hidden || fnb.starts_with(&['.' as u8]),
                    };
                    other_entries.push(ent);
                }
                continue;
            }
            
            dir_q.push(val.path());
        }
    }

    return Ok(dir_q.drain(dIdx..).collect());
}