use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use regex::bytes::Regex;
use std::ffi::OsStr;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::Config;
use crate::walk::walk_match_until_limit;

#[derive(Clone, Debug)]
pub struct FoundFile {
    pub s_path: String,
    pub is_symlink: bool,
    pub is_hidden: bool,
}

pub fn find(target: String, root: std::path::PathBuf, cfg: &Config) -> Result<Vec<String>, Error> {
    if cfg.num_threads < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("invalid number of threads, '-t' MUST be >= 2")))
    }
    
    // Filter applied if provided arguments to hide: hidden files, symlinks
    let mut maybe_filter: Option<(fn(a: &FoundFile, cfg: &Config) -> bool, Config)> = None;
    if !cfg.show_hidden || !cfg.show_symlinks {
        maybe_filter = Some((|a: &FoundFile, cfg: &Config| {
            return (cfg.show_hidden || !a.is_hidden) && 
                   (cfg.show_symlinks || !a.is_symlink);
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
    let maybe_initial_paths = walk_match_until_limit(vec![root], 2 * cfg.num_threads, regex_target.clone(), exact_match_target);
    let Ok((mut paths_to_distribute, mut all_results)) = maybe_initial_paths else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read root path: {:?}", maybe_initial_paths.err())))
    };
    if !cfg.sorted {
        print_walk_results(&all_results);
    }

    // Distribute paths between threads s.t. threads spawned later get some "low depth" paths
    // EXAMPLE: all_paths_len = 24, num_threads = 8 -> thread_0_paths = [all_paths[0], all_paths[8], all_paths[16]]
    let max_num_paths_per_thread = (paths_to_distribute.len() / cfg.num_threads) + 1;
    let mut paths_per_thread: Vec<Vec<PathBuf>> = vec![Vec::with_capacity(max_num_paths_per_thread); cfg.num_threads];
    let mut chunk_size = cfg.num_threads;
    while paths_to_distribute.len() > 0 {
        if chunk_size >= paths_to_distribute.len() {
            chunk_size = paths_to_distribute.len();
        }
        let chunk: Vec<PathBuf> = paths_to_distribute.drain(0..chunk_size).collect();
        
        for j in 0..chunk.len() {
            paths_per_thread[j].push(chunk[j].clone());
        }
    }

    // Main thread loop
    loop {
        // Start "walk" on auxiliary threads
        let new_dirs_and_results: (Vec<Vec<PathBuf>>, Vec<Vec<FoundFile>>) = paths_per_thread.par_iter_mut().map(|p| {
            let Ok((maybe_send_to_main, mut thread_results)) = walk_match_until_limit(p.to_vec(), cfg.file_dir_limit, regex_target.clone(), exact_match_target) 
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
        
        // "Fair" distribution of paths doesn't matter as much in the main loop, so paths are assigned to threads sequentially here
        let mut curr_num_threads = cfg.num_threads;
        if paths_to_distribute.len() < curr_num_threads {
            curr_num_threads = paths_to_distribute.len();
        }
        paths_per_thread = Vec::with_capacity(curr_num_threads);
        let min_paths_per_thread = paths_to_distribute.len() / curr_num_threads;
        let mut rem_paths = paths_to_distribute.len() - (min_paths_per_thread * curr_num_threads);
        while paths_to_distribute.len() > 0 {
            let mut num_thread_paths = min_paths_per_thread;
            if rem_paths > 0 {
                num_thread_paths += 1;
                rem_paths -= 1;
            }
            
            let paths = paths_to_distribute.drain(0..num_thread_paths).collect();
            paths_per_thread.push(paths);
        }
    }
    
    // Not sorted -> Threads handle printing so nothing to return
    if !cfg.sorted {
        return Ok(Vec::new())
    }

    all_results.sort_by(|a, b| {
        return a.s_path.cmp(&b.s_path);
    });
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