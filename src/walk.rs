use regex::bytes::Regex;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::label::add_label;
use crate::matches::{initialise_matches_capacities, insert_entry_in_matches, NUM_FILE_CATEGORIES};

const HIDDEN_RX_STR: &str = r".*\/\..*";

pub fn walk_match_until_limit_file_names(initial_dirs: &mut Vec<std::path::PathBuf>, limit: usize, label_pos: i8, match_rx: Regex, match_exact: Option<&String>) -> std::io::Result<(Vec<PathBuf>, [Vec<String>; NUM_FILE_CATEGORIES])> {
    let mut dir_q: Vec<PathBuf> = std::mem::take(initial_dirs);
    let mut match_exact_fn = Some(OsStr::new(""));
    if match_exact.is_some() {
        match_exact_fn = Some(OsStr::new(match_exact.unwrap()));
    }
    
    // Actual limit should be min(limit, some.len())
    let mut fd_limit = limit;
    if limit < dir_q.len() {
        fd_limit = dir_q.len();
    }
    let mut matches: [Vec<String>; NUM_FILE_CATEGORIES] = initialise_matches_capacities(fd_limit);
    
    let mut f_idx = 0;
    let mut d_idx = 0;
    let hidden_rx = Regex::new(&HIDDEN_RX_STR).unwrap();
    let is_match_exact = match_exact.is_some();
    while (f_idx + d_idx) < fd_limit && d_idx < dir_q.len() {
        let dir_name = dir_q[d_idx].file_name();
        let parent_path_os_str = dir_q[d_idx].to_path_buf().into_os_string();
        let parent_hidden = hidden_rx.is_match(parent_path_os_str.as_bytes());
        if (is_match_exact && dir_name == match_exact_fn) || 
           (!is_match_exact && match_rx.is_match(dir_name.unwrap().as_bytes())) {
            // Add trailing '/' to dir paths to differentiate them
            let mut parent_path_string = parent_path_os_str.into_string().unwrap();
            parent_path_string.push('/');
            insert_entry_in_matches(&mut matches, parent_path_string, parent_hidden, false, false);
        }
        
        let dir_entries = std::fs::read_dir(&dir_q[d_idx])?;
        d_idx += 1;
        for ent in dir_entries {
            let Ok(val) = ent else { continue };
            let Ok(ft) = val.file_type() else { continue };
            f_idx += 1;
            
            if ft.is_file() || ft.is_symlink() {
                let file_name = val.file_name();
                if (is_match_exact && file_name.as_os_str() == match_exact_fn.unwrap()) || 
                   (!is_match_exact && match_rx.is_match(file_name.as_bytes())) {
                    let file_path_string = val.path().into_os_string().into_string().unwrap();
                    insert_entry_in_matches(&mut matches, file_path_string, parent_hidden || file_name.as_bytes().starts_with(&['.' as u8]), true, ft.is_symlink());
                }
                continue;
            }

            dir_q.push(val.path());
        }
    }

    let is_labelled = label_pos != 0;
    if is_labelled {
        for i in 0..matches.len() {
            let is_hidden = i >= 3;
            let file_type_idx = i % 3;
            for j in 0..matches[i].len() {
                matches[i][j] = add_label(&mut matches[i][j], label_pos, is_hidden, file_type_idx == 0, file_type_idx == 1);
            }
        }
    }

    return Ok((dir_q.drain(d_idx..).collect(), matches));
}