use regex::bytes::Regex;
use std::ffi::OsStr;
use std::fs;
use std::io::BufRead;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::find::FoundFile;

const HIDDEN_RX_STR: &str = r".*\/\..*";

pub fn walk_match_until_limit(initial_dirs: &mut Vec<std::path::PathBuf>, limit: usize, label_pos: i8, contents_search: bool, match_rx: Regex, match_exact: Option<&String>) -> std::io::Result<(Vec<PathBuf>, Vec<Vec<Vec<FoundFile>>>)> {
    if !contents_search {
        return walk_match_until_limit_file_names(initial_dirs, limit, label_pos, match_rx, match_exact);
    }
    return walk_match_until_limit_contents(initial_dirs, limit, match_rx, match_exact)
}

fn walk_match_until_limit_file_names(initial_dirs: &mut Vec<std::path::PathBuf>, limit: usize, label_pos: i8, match_rx: Regex, match_exact: Option<&String>) -> std::io::Result<(Vec<PathBuf>, Vec<Vec<Vec<FoundFile>>>)> {
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
    let mut matches: Vec<Vec<Vec<FoundFile>>> = initialise_matches_capacities(fd_limit);
    
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
            let ent = FoundFile {
                s_path: parent_path_string,
                maybe_lines: None,
            };
            insert_entry_in_matches(&mut matches, ent, parent_hidden, false, false);
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
                    let ent = FoundFile {
                        s_path: file_path_string,
                        maybe_lines: None
                    };
                    insert_entry_in_matches(&mut matches, ent, parent_hidden || file_name.as_bytes().starts_with(&['.' as u8]), true, ft.is_symlink());
                }
                continue;
            }

            dir_q.push(val.path());
        }
    }

    let is_labelled = label_pos != 0;
    if is_labelled {
        for i in 0..2 {
            let is_hidden = i == 1;
            for j in 0..3 {
                for ent in &mut matches[i][j] {
                    ent.s_path = add_label(&ent.s_path, label_pos, is_hidden, j == 0, j == 1);
                }
            }
        }
    }

    return Ok((dir_q.drain(d_idx..).collect(), matches));
}

fn add_label(s_path: &String, label_pos: i8, is_hidden: bool, is_file: bool, is_symlink: bool) -> String {
    let label = generate_label(is_hidden, is_file, is_symlink);
    if label_pos == 1 {
        return format!("{} {}", s_path, label);
    }
    return format!("{} {}", label, s_path);
}

fn generate_label(is_hidden: bool, is_file: bool, is_symlink: bool) -> String {
    let mut ret = String::from("FRR");
    if !is_file {
        ret.replace_range( 0..2, "D_");
    }
    if is_symlink {
        ret.replace_range( 1..2, "S");
    }
    if is_hidden {
        ret.replace_range( 2..3, "H");
    }
    return ret;
}

fn walk_match_until_limit_contents(initial_dirs: &mut Vec<std::path::PathBuf>, limit: usize, match_rx: Regex, match_exact: Option<&String>) -> std::io::Result<(Vec<PathBuf>, Vec<Vec<Vec<FoundFile>>>)> {
    let mut dir_q: Vec<PathBuf> = std::mem::take(initial_dirs);
    
    // Actual limit should be min(limit, some.len())
    let mut fd_limit = limit;
    if limit < dir_q.len() {
        fd_limit = dir_q.len();
    }
    let mut matches: Vec<Vec<Vec<FoundFile>>> = initialise_matches_capacities(fd_limit);
    
    let mut f_idx = 0;
    let mut d_idx = 0;
    let hidden_rx = Regex::new(&HIDDEN_RX_STR).unwrap();
    let is_match_exact = match_exact.is_some();
    while (f_idx + d_idx) < fd_limit && d_idx < dir_q.len() {
        let parent_path_os_str = dir_q[d_idx].to_path_buf().into_os_string();
        let parent_hidden = hidden_rx.is_match(parent_path_os_str.as_bytes());
        
        let dir_entries = std::fs::read_dir(&dir_q[d_idx])?;
        d_idx += 1;
        for ent in dir_entries {
            let Ok(val) = ent else { continue };
            let Ok(ft) = val.file_type() else { continue };
            f_idx += 1;
            
            if ft.is_file() || ft.is_symlink() {
                let mut file_path = val.path();
                let mut file_name = val.file_name();
                if ft.is_symlink() {
                    let maybe_path = fs::canonicalize(file_path.clone());
                    if maybe_path.is_err() {
                        // TODO: Capture error?
                        continue;
                    }

                    file_path = maybe_path.unwrap();
                    file_name = file_path.file_name().unwrap().to_os_string();
                }
                
                let file= std::fs::File::open(&file_path);
                if file.is_err() {
                    // TODO: Capture error?
                    continue;
                }

                let lines: Vec<Result<String, std::io::Error>> = std::io::BufReader::new(file.unwrap()).lines().collect();
                let mut line_matches: Vec<String> = Vec::with_capacity(lines.len());
                let mut line_num = 0;
                for l in lines {
                    line_num += 1;
                    if l.is_err() {
                        continue;
                    }

                    let line = l.unwrap();
                    if (is_match_exact && &line == match_exact.unwrap()) || 
                       (!is_match_exact && match_rx.is_match(line.as_bytes())) {
                        line_matches.push(format!("{}:{}", line_num, line));
                    }
                }

                let file_path_string = file_path.into_os_string().into_string().unwrap();
                if line_matches.len() > 0 {
                    let ent = FoundFile {
                        s_path: file_path_string,
                        maybe_lines: Some(line_matches)
                    };
                    insert_entry_in_matches(&mut matches, ent, parent_hidden || file_name.as_bytes().starts_with(&['.' as u8]), true, ft.is_symlink());
                }

                continue;
            }

            dir_q.push(val.path());
        }
    }

    return Ok((dir_q.drain(d_idx..).collect(), matches));
}

fn insert_entry_in_matches(matches: &mut Vec<Vec<Vec<FoundFile>>>, ent: FoundFile, hidden: bool, file: bool, symlink: bool) {
    let is_hidden_idx = hidden as usize;
    let entry_type_idx = (symlink as usize) + ((!file as usize) * 2);
    matches[is_hidden_idx][entry_type_idx].push(ent);
}

// initialise_matches_capacities, sets the capacities of all 3 "levels" of the vector based on a guess of the occurences of:
// not hidden [0][_], hidden [1][_], files [_][0], symlinks [_][1] and directories [_][2]
fn initialise_matches_capacities(fd_limit: usize) -> Vec<Vec<Vec<FoundFile>>> {
    let mut matches = vec![Vec::with_capacity(3), Vec::with_capacity(3)];
    let mut left = fd_limit;
    // TODO: These multipliers are based on the sample folder used for BENCHMARKS 
    //       Could probably determine a better estimate for these...
    let type_mults = vec![92.5/100.0, 1.0/100.0, 6.5/100.0];
    let hidden_mults = vec![99.9/100.0, 0.1/100.0];
    for i in 0..3 {
        let n_hidden_cap = ((fd_limit as f64) * type_mults[i] * hidden_mults[0]).floor() as usize;
        let hidden_cap = ((fd_limit as f64) * type_mults[i] * hidden_mults[1]).floor() as usize;
        matches[0].push(Vec::with_capacity(n_hidden_cap));
        matches[1].push(Vec::with_capacity(hidden_cap));
        left -= hidden_cap + n_hidden_cap;

        if i == 2 {
            matches[0][i] = Vec::with_capacity(matches[1][i].capacity() + left);
        }
    }
    return matches;
}