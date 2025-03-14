use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use regex::bytes::Regex;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::Config;
use crate::walk::walk_match_until_limit;

#[derive(Clone, Debug)]
pub struct FoundFile {
    pub s_path: String,
    pub is_file: bool,
    pub is_symlink: bool,
    pub is_hidden: bool,
}

const FIRST_WALK_LIMIT: usize = 256;
const SORT_ASC: fn(a: &FoundFile, b: &FoundFile) -> Ordering = |a: &FoundFile, b: &FoundFile| {
    return a.s_path.cmp(&b.s_path);
};
const SORT_DESC: fn(a: &FoundFile, b: &FoundFile) -> Ordering = |a: &FoundFile, b: &FoundFile| {
    return a.s_path.cmp(&b.s_path).reverse();
};

pub fn find(target: String, root: std::path::PathBuf, cfg: &Config) -> Result<Vec<String>, Error> {
    if cfg.num_threads < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("invalid number of threads, '-t' MUST be >= 2")))
    }
    
    // Filter applied if provided arguments to hide: files, directories, symlinks or hidden items
    // TODO: "show ONLY hidden" and "show ONLY symlink" conditions are currently broken
    let mut maybe_filter: Option<(fn(a: &FoundFile, cfg: &Config) -> bool, Config)> = None;
    if !cfg.show_files || !cfg.show_dirs || !cfg.show_symlinks || !cfg.show_hidden {
        maybe_filter = Some((|a: &FoundFile, cfg: &Config| {
            return  (cfg.show_files || !a.is_file) &&
                    (cfg.show_dirs || a.is_file) &&
                    (cfg.show_symlinks || !a.is_symlink) &&
                    (cfg.show_hidden || !a.is_hidden);
        }, cfg.clone()))
    }
    
    // Set variables for regex OR exact match based on config
    let mut regex_target = Regex::new("").unwrap();
    let mut exact_match_target = Some(OsStr::new(&target));
    if !cfg.equality_match {
        let maybe_match = Regex::new(&target);
        if maybe_match.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to compile regex")));
        }
        regex_target = maybe_match.unwrap();
        exact_match_target = None;
    }
    
    // Find multiple directory paths from `root`, to distribute them between threads later
    let mut initial_dirs = vec![root];
    let maybe_initial_paths = walk_match_until_limit(&mut initial_dirs, FIRST_WALK_LIMIT, regex_target.clone(), exact_match_target);
    let Ok((mut paths_to_distribute, mut all_results)) = maybe_initial_paths else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read root path: {:?}", maybe_initial_paths.err())))
    };
    if !cfg.sorted {
        print_walk_results(&all_results);
    }

    // Main thread loop
    loop {
        // Redistribute paths
        let mut curr_num_threads = cfg.num_threads;
        if paths_to_distribute.len() < curr_num_threads {
            curr_num_threads = paths_to_distribute.len();
        }
        let mut paths_per_thread = distribute_paths_per_thread(&mut paths_to_distribute, curr_num_threads);

        // Start "walk" on auxiliary threads
        let new_dirs_and_results: (Vec<Vec<PathBuf>>, Vec<Vec<FoundFile>>) = paths_per_thread.par_iter_mut().map(|p| {
            let Ok((maybe_send_to_main, mut thread_results)) = walk_match_until_limit(p, cfg.file_dir_limit, regex_target.clone(), exact_match_target) 
            else {
                return (vec![], vec![]);
            };
            
            // All filtering is handled in auxiliary threads
            if maybe_filter.is_some() {
                let (filter_fn, cfg): (fn(a: &FoundFile, cfg: &Config) -> bool, Config)  = maybe_filter.clone().unwrap();
                thread_results = thread_results.into_iter().filter(|it| {
                    return filter_fn(it, &cfg)
                }).collect();
            }

            // Not sorted -> Can handle printing in threads and "drop" results
            if !cfg.sorted {
                print_walk_results(&thread_results);
                return (maybe_send_to_main, Vec::new());
            }
            
            return (maybe_send_to_main, thread_results)
        }).unzip();

        // Retrieve paths to distribute and add to all_results    
        paths_to_distribute = new_dirs_and_results.0.into_iter().flatten().collect();
        all_results.append(&mut new_dirs_and_results.1.into_iter().flatten().collect());
        if paths_to_distribute.len() == 0 {
            break;
        }
    }
    
    // Not sorted -> Threads handle printing so nothing to return
    if !cfg.sorted {
        return Ok(Vec::new())
    }

    if cfg.sort_asc {
        all_results.par_sort_by(SORT_ASC);
    } else {
        all_results.par_sort_by(SORT_DESC);
    }
    let result_strings = all_results.into_iter().map(|s|{
        return s.s_path
    }).collect();
    return Ok(result_strings);
}

fn print_walk_results(results: &Vec<FoundFile>) {
    if results.len() == 0 {
        return;
    }

    let mut lines: Vec<String> = Vec::with_capacity(results.len());
    for r in results.clone() {
        lines.push(r.s_path);
    }
    let output_str = format!("{}\n", lines.join("\n"));
    let _ = std::io::stdout().write(output_str.as_bytes());
}

fn distribute_paths_per_thread(paths_to_distribute_and_free: &mut Vec<PathBuf>, num_threads: usize) -> Vec<Vec<PathBuf>> {
    // distribute paths such that each thread gets a "fair" allocation of low and high index elements
    let max_num_paths_per_thread = (paths_to_distribute_and_free.len() / num_threads) + 1;
    let mut paths_per_thread: Vec<Vec<PathBuf>> = vec![Vec::with_capacity(max_num_paths_per_thread); num_threads];
    for i in 0..num_threads {
        for j in 0..max_num_paths_per_thread {
            let take_idx = (j * num_threads) + i;
            if take_idx >= paths_to_distribute_and_free.len() {
                break;
            }
            paths_per_thread[i].push(paths_to_distribute_and_free[take_idx].clone());
        }
    }
    
    // the original data is no longer needed, free it
    paths_to_distribute_and_free.clear();
    paths_to_distribute_and_free.shrink_to_fit();

    return paths_per_thread;
}