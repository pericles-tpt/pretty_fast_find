use std::{collections::HashSet, io::Error};

use crate::{utility::thread_from_root_new, walk::walk_search_until_limit};

// t: 0 -> dir, 1 -> dir, 2 -> symlink, 3 -> other
#[derive(Debug, Clone)]
pub struct FoundFile {
    pub p: String,
    pub is_sym: bool,
    pub is_hidden: bool,
}

pub fn find(target_substring: String, target_path: std::path::PathBuf, num_threads: usize, thread_add_dir_limit: usize, show_hidden: bool, sorted: bool) -> Result<Vec<String>, Error> {
    if num_threads <= 1 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Provided 0 or 1 number of threads, must provide '-t' argument > 1")))
    }
    
    let mut maybe_filter: Option<fn(a: &FoundFile) -> bool> = None;
    if !show_hidden {
        maybe_filter = Some(|a: &FoundFile| {
            return !a.is_hidden;
        })
    }
    
    let maybe_curr_scan: std::io::Result<Vec<FoundFile>> = thread_from_root_new(
        target_path, 
        HashSet::new(),
        &target_substring, 
        num_threads, 
        thread_add_dir_limit, 
        None, 
        Some(walk_search_until_limit), 
        |a, b| {
            return a.p.cmp(&b.p);
        },
        maybe_filter,
        sorted,
        |a| {
            a.p
        }
    );
    
    if maybe_curr_scan.is_err() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to do MT find: {:?}", maybe_curr_scan.err())))
    }
    
    if !sorted {
        return Ok(Vec::new())
    }

    let mut curr_scan = maybe_curr_scan.unwrap();
    curr_scan.sort_by(|a, b| {
        return a.p.cmp(&b.p);
    });
    
    let ret = curr_scan.into_iter().map(|s|{
        return s.p
    }).collect();

    return Ok(ret);
}