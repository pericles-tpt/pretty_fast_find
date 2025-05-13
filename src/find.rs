use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use regex::bytes::Regex;
use std::io::{Error, Write};
use std::path::PathBuf;

use crate::label;
use crate::matches;
use crate::walk;
use crate::Config;

const FIRST_WALK_FDL: usize = 256;

const FT_FILE: usize = 0;
const FT_SYMLINK: usize = 1;
const FT_DIR: usize = 2;

pub fn find(target: String, root: std::path::PathBuf, cfg: &Config) -> Result<Vec<String>, Error> {
    if cfg.num_threads < 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("invalid number of threads, '-t' MUST be >= 2")))
    }

    // Set variables for regex OR exact match based on config
    let mut regex_target = Regex::new("").unwrap();
    let mut exact_match_target = Some(&target);
    if !cfg.equality_match {
        let regex_result = Regex::new(&target);
        if regex_result.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to compile regex")));
        }
        regex_target = regex_result.unwrap();
        exact_match_target = None;
    }
    
    // Find multiple directory paths from `root`, to distribute them between threads later
    let mut initial_dirs = vec![root];
    let maybe_initial_paths = walk::walk_collect_matches_until_limit(&mut initial_dirs, FIRST_WALK_FDL, cfg.label_pos, regex_target.clone(), exact_match_target);
    let Ok((mut paths_to_distribute, mut categorised_results)) = maybe_initial_paths else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read root path: {:?}", maybe_initial_paths.err())))
    };

    let mut flat_results = filter_elements(cfg,&mut categorised_results).into_iter().flatten().collect();
    
    if !cfg.is_sorted {
        print_walk_results(&flat_results);
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
        let new_dirs_and_results: (Vec<Vec<PathBuf>>, Vec<Vec<String>>) = paths_per_thread.par_iter_mut().map(|paths| {
            let Ok((thread_paths_to_distribute, mut thread_categorised_results)) = walk::walk_collect_matches_until_limit(paths, cfg.file_dir_limit, cfg.label_pos, regex_target.clone(), exact_match_target) 
            else {
                return (vec![], vec![]);
            };
            
            // All filtering is handled in auxiliary threads
            let thread_flat_results = filter_elements(cfg, &mut thread_categorised_results).into_iter().flatten().collect();

            // Not sorted -> Can print immediately and "drop" results here
            if !cfg.is_sorted {
                print_walk_results(&thread_flat_results);
                return (thread_paths_to_distribute, Vec::new());
            }
            
            return (thread_paths_to_distribute, thread_flat_results)
        }).unzip();

        // Retrieve paths to distribute and add to all_results    
        paths_to_distribute = new_dirs_and_results.0.into_iter().flatten().collect();
        flat_results.append(&mut new_dirs_and_results.1.into_iter().flatten().collect());
        let finished_walk = paths_to_distribute.len() == 0;
        if finished_walk {
            break;
        }
    }
    
    // Not sorted -> Threads handle printing so nothing to return
    if !cfg.is_sorted {
        return Ok(Vec::new())
    }

    // Custom start/end offsets are used when comparing paths, to ensure labels AREN'T included
    let mut start_cmp_str_offset = 0;
    let mut end_cmp_str_offset = 0;
    if cfg.label_pos != 0 {
        start_cmp_str_offset = label::LABEL_LENGTH + 1;
        if cfg.label_pos > 0 {
            start_cmp_str_offset = 0;
            end_cmp_str_offset = label::LABEL_LENGTH + 1;
        }
    }
    if cfg.sort_asc {
        flat_results.par_sort_by(|a: &String, b: &String| {
            return a[start_cmp_str_offset..a.len() - end_cmp_str_offset].cmp(&b[start_cmp_str_offset..b.len() - end_cmp_str_offset]);
        });
    } else {
        flat_results.par_sort_by(|a: &String, b: &String| {
            return a[start_cmp_str_offset..a.len() - end_cmp_str_offset].cmp(&b[start_cmp_str_offset..b.len() - end_cmp_str_offset]).reverse();
        });
    }
    return Ok(flat_results);
}

fn print_walk_results(results: &Vec<String>) {
    if results.len() == 0 {
        return;
    }

    let mut lines: Vec<String> = Vec::with_capacity(results.len());
    for ff in results {
        lines.push(ff.clone());
    }
    let output_str = format!("{}\n", lines.join("\n"));
    let _ = std::io::stdout().write(output_str.as_bytes());
}

fn distribute_paths_per_thread(paths_to_distribute_and_free: &mut Vec<PathBuf>, num_threads: usize) -> Vec<Vec<PathBuf>> {
    // distribute paths such that each thread gets a "fair" allocation of low and high index elements
    let max_num_paths_per_thread = (paths_to_distribute_and_free.len() / num_threads) + 1;
    let mut per_thread_paths: Vec<Vec<PathBuf>> = vec![Vec::with_capacity(max_num_paths_per_thread); num_threads];
    for i in 0..num_threads {
        for j in 0..max_num_paths_per_thread {
            let take_idx = (j * num_threads) + i;
            if take_idx >= paths_to_distribute_and_free.len() {
                break;
            }
            per_thread_paths[i].push(paths_to_distribute_and_free[take_idx].clone());
        }
    }
    
    // the original data is no longer needed, free it
    paths_to_distribute_and_free.clear();
    paths_to_distribute_and_free.shrink_to_fit();

    return per_thread_paths;
}

// filter_elements, determines which indices in the Vec<Vec<FoundFile>> to retrieve based on filters in config
fn filter_elements(cfg: &Config, original: &mut [Vec<String>; matches::NUM_FILE_CATEGORIES]) -> Vec<Vec<String>> {
    let mut filtered_hidden = vec![0, 1];
    let mut filtered_types = vec![FT_FILE, FT_SYMLINK, FT_DIR];
    if cfg.is_filtered {
        if cfg.filter_hidden {
            filtered_hidden = vec![cfg.show_hidden as usize];
        }

        filtered_types = Vec::with_capacity(3);
        if cfg.show_files {
            filtered_types.push(FT_FILE);
            if cfg.filter_symlinks {
                if cfg.show_symlinks {
                    filtered_types[0] = FT_SYMLINK;
                }
            } else {
                filtered_types.push(FT_SYMLINK);
            }
        }
        if cfg.show_dirs && (!cfg.filter_symlinks || !cfg.show_symlinks) {
            filtered_types.push(FT_DIR);
        }
    }
    
    let mut ret = Vec::with_capacity(filtered_hidden.len() * filtered_types.len());
    for is_hidden in filtered_hidden {
        for ft in &filtered_types {
            let filtered_idx = (is_hidden as usize * 3) + *ft;
            ret.push(std::mem::take(&mut original[filtered_idx]));
        }
    }
    return ret;
}