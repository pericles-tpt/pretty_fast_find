use std::{io::{Error, Write}, path::PathBuf};
use rayon::iter::{ParallelIterator, IntoParallelRefMutIterator};
use crate::walk::walk_search_until_limit;

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
    
    let mut res: Vec<FoundFile> = Vec::new();

    // Do first pass of thread_*_fn() on root to get multiple items
    let mut initial_input = vec![target_path];
    let maybe_initial_items = walk_search_until_limit(&target_substring, initial_input, &mut res, 2 * num_threads);
    let Ok(mut paths_to_distribute) = maybe_initial_items else {return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read root path: {:?}", maybe_initial_items.err())))};
    if !sorted {
        let mut lines: Vec<String> = Vec::with_capacity(res.len());
        for r in res.clone() {
            lines.push(r.p);
        }
        if lines.len() > 0 {
            let output_str = format!("{}\n", lines.join("\n"));
            let res = std::io::stdout().write(output_str.as_bytes());
        }
    }

    // Interleave items in paths_to_distribute for better distribution
    // Swap every 2nd item
    let vc = (paths_to_distribute.len() / num_threads) + 1;
    let mut paths_to_distribute_per_thread: Vec<Vec<PathBuf>> = vec![Vec::with_capacity(vc); num_threads];
    let mut chunk_size = num_threads;
    while paths_to_distribute.len() > 0 {
        if chunk_size >= paths_to_distribute.len() {
            chunk_size = paths_to_distribute.len();
        }
        let chunk: Vec<PathBuf> = paths_to_distribute.drain(0..chunk_size).collect();
        
        for j in 0..chunk.len() {
            paths_to_distribute_per_thread[j].push(chunk[j].clone());
        }
    }
    
    loop {
        let prs: Vec<(Vec<PathBuf>, Vec<FoundFile>)> = paths_to_distribute_per_thread.par_iter_mut().map(|p| {
            let mut results: Vec<FoundFile> = Vec::new();
            
            let maybe_send_to_main = walk_search_until_limit(&target_substring, p.to_vec(), &mut results, thread_add_dir_limit);
            
            if maybe_filter.is_some() {
                let filter_fn = maybe_filter.unwrap();
                results = results.into_iter().filter(|it| {
                    return filter_fn(it)
                }).collect();
            }
            if maybe_send_to_main.is_err() {
                return (vec![], results);
            }
            
            if sorted {
                return (maybe_send_to_main.unwrap(), results)
            }
            
            let mut lines: Vec<String> = Vec::with_capacity(results.len());
            for r in results {
                lines.push(r.p);
            }
            if lines.len() > 0 {
                let output_str = format!("{}\n", lines.join("\n"));
                let res = std::io::stdout().write(output_str.as_bytes());
            }
            return (maybe_send_to_main.unwrap(), Vec::new());
        }).collect();
        
        let split_pair: (Vec<Vec<PathBuf>>, Vec<Vec<FoundFile>>) = prs.into_iter().map(|(a, b)|(a, b)).unzip();
        let mut new_results = split_pair.1.into_iter().flatten().collect();
        paths_to_distribute = split_pair.0.into_iter().flatten().collect();
        res.append(&mut new_results);
        
        if paths_to_distribute.len() == 0 {
            break;
        }
        
        // Split `paths_to_distribute` s.t. each thread operates on a (roughly) equal number of paths
        let mut curr_num_threads = num_threads;
        if paths_to_distribute.len() < curr_num_threads {
            curr_num_threads = paths_to_distribute.len();
        }
        paths_to_distribute_per_thread = Vec::with_capacity(curr_num_threads);
        
        let min_paths_per_thread = paths_to_distribute.len() / curr_num_threads;
        let mut rem_paths = paths_to_distribute.len() - (min_paths_per_thread * curr_num_threads);
        while paths_to_distribute.len() > 0 {
            let mut num_thread_paths = min_paths_per_thread;
            if rem_paths > 0 {
                num_thread_paths += 1;
                rem_paths -= 1;
            }
            
            let paths = paths_to_distribute.drain(0..num_thread_paths).collect();
            paths_to_distribute_per_thread.push(paths);
        }
    }
    
    if !sorted {
        return Ok(Vec::new())
    }

    res.sort_by(|a, b| {
        return a.p.cmp(&b.p);
    });
    
    let ret = res.into_iter().map(|s|{
        return s.p
    }).collect();

    return Ok(ret);
}