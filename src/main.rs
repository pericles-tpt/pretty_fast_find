use std::env;
use std::io::Write;
use std::num::ParseIntError;
use std::path::PathBuf;

mod find;
mod walk;
mod matches;
mod label;

const DEFAULT_NUM_THREADS: usize = 84;
const DEFAULT_FD_LIMIT: usize = 2048;

struct Config {
    num_threads: usize,
    file_dir_limit: usize,
    include_target_in_output: bool,
    show_files: bool,
    show_dirs: bool,
    show_symlinks: bool,
    filter_symlinks: bool,
    show_hidden: bool,
    filter_hidden: bool,
    is_filtered: bool,
    is_sorted: bool,
    sort_asc: bool,
    label_pos: i8, // -1 -> start, 0 -> none, 1 -> end
    equality_match: bool,
}

fn main() {
    let mut cfg = Config {
        num_threads:              DEFAULT_NUM_THREADS,
        file_dir_limit:           DEFAULT_FD_LIMIT,
        include_target_in_output: false,
        show_files:               true,
        show_dirs:                true,
        show_symlinks:            true,
        filter_symlinks:          false,
        show_hidden:              true,
        filter_hidden:            false,
        is_filtered:              false,
        is_sorted:                false,
        sort_asc:                 true,
        label_pos:                0,
        equality_match:           false,
    };

    let (target, root);
    match eval_args(&env::args().skip(1).collect(), &mut cfg) {
        Ok(required_args) => {
            target = required_args.0;
            root = required_args.1;
            if target.len() == 0 {
                return;
            }
        }
        Err(e ) => {
            eprintln!("error: {}", e);
            return;
        }
    }

    match find::find(target, root, &cfg) {
        Ok(entries) => {
            if entries.len() == 0 {
                return;
            }
            
            let res = std::io::stdout().write(format!("{}\n", entries.join("\n")).as_bytes());
            if res.is_err() {
                eprintln!("failed to write `find` results to stdout: {:?}", res.err());
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}


fn eval_args(args: &Vec<String>, config: &mut Config) -> std::io::Result<(String, PathBuf)> {
    // Length Checks / Help Output
    let default_ret = (String::new(), PathBuf::new());
    if args.len() == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "insufficient arguments for `pff`, expected: `pff --help`, `pff --version` or `pff [PATTERN] [ROOT FIND DIRECTORY]`"));
    }
    if args.len() == 1 {
        match args[0].as_str() {
            "--help" => {
                print_help_text();
            }
            "--version" => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            }
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "insufficient arguments for `pff`, expected: `pff --help`, `pff --version` or `pff [PATTERN] [ROOT FIND DIRECTORY]`"));
            }
        }
        return Ok(default_ret);
    }

    // Required Args
    let target = args[args.len() - 2].to_string();
    let root = &args[args.len() - 1];
    if !std::fs::exists(root)? {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("provided path '{}', does not exist", root)));
    }
    let root_pb = PathBuf::from(root);
    let has_optional_args = args.len() > 2;
    if !has_optional_args {
        return Ok((target, root_pb));
    }

    // Optional Args
    let mut i = 0;
    let first_non_optional_arg_idx = args.len() - 2;
    let valid_command_options = vec!["--include-target", "-eq", "--filter", "--sort", "--label", "-t", "-fdl"];
    while i < first_non_optional_arg_idx {
        let curr = args[i].as_str();
        if !valid_command_options.contains(&curr) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid argument '{}', must be one of: {}", curr, valid_command_options.join(", "))));
        }
    
        // Toggle Args (no additional args)
        let mut is_valid_opt = true;
        match curr {
            "--include-target" =>{
                config.include_target_in_output = true;
            }
            "-eq" => {
                config.equality_match = true;
            }
            _ => { is_valid_opt = false; }
        }
        i += 1;
        if is_valid_opt {
            continue;
        }
        
        // Multi-value args
        if i + 1 >= args.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("missing additional argument for '{}' flag", curr)));
        }
        let mut next = args[i].as_str();
        match curr {
            "-t" => {
                let maybe_num_threads: Result<usize, ParseIntError> = next.parse();
                if maybe_num_threads.is_err() || maybe_num_threads.clone().unwrap() < 1 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid `-t` argument, must be a non-negative integer"));
                }
                config.num_threads = maybe_num_threads.unwrap();
            }
            "--filter" => {
                let valid_filter_options = ["f", "d", "s", "h", "nf", "nd", "ns", "nh"];
                config.is_filtered = true;

                // Duplicate counter
                let mut fc = 0;
                let mut dc = 0;
                let mut sc = 0;
                let mut hc = 0;
                
                while valid_filter_options.contains(&next) {
                    let mut is_show = true;
                    if next.starts_with("n") {
                        is_show = false;
                        next = &next[1..];
                    }
                    
                    match next {
                        "f" => {
                            config.show_files = is_show;
                            config.show_dirs = !is_show;
                            fc += 1;
                        }
                        "d" => {
                            config.show_dirs = is_show;
                            config.show_files = !is_show;
                            dc += 1;
                        }
                        "s" => {
                            config.show_symlinks = is_show;
                            config.filter_symlinks = true;
                            sc += 1;
                        }
                        "h" => {
                            config.show_hidden = is_show;
                            config.filter_hidden = true;
                            hc += 1;
                        }
                        _ => {
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid option: '{}', provided for --filter, must be one of: {}", next, valid_filter_options.join(", "))));
                        }
                    }

                    i += 1;
                    if i >= args.len() {
                        break;
                    }
                    next = args[i].as_str();
                }
                let stopped_before_last_optional_arg = i < args.len();
                if stopped_before_last_optional_arg {
                    i -= 1;
                }
                
                if fc > 1 || dc > 1 || sc > 1 || hc > 1 {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid filter parameter, you cannot provide two or more of the same filter option"));
                } else if fc > 0 && dc > 0 {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid filter parameter, only one of 'd' and 'f' can be provided at a time"));
                }
            }
            "--sort" => {
                let mut is_asc = true;
                if next == "desc" {
                    is_asc = false;
                } else if next != "asc" {
                    i -= 1;
                }
                config.is_sorted = true;
                config.sort_asc = is_asc;
            }
            "--label" => {
                let mut pos = -1;
                if next == "end" {
                    pos = 1;
                } else if next != "start" {
                    i -= 1;
                }
                config.label_pos = pos;
            }
            "-fdl" => {
                let maybe_file_dir_limit: Result<usize, ParseIntError> = next.parse();
                if maybe_file_dir_limit.is_err() || maybe_file_dir_limit.clone().unwrap() < 1 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid `-fdl` limit argument, must be a non-negative integer"));
                }
                config.file_dir_limit = maybe_file_dir_limit.unwrap();
            }
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unimplemented arg: '{}'", curr)));
            }
        }
        i += 1;
    }
 
    Ok((target, root_pb))    
}

fn print_help_text() {
    println!("Pretty Fast Find, finds items in your filesystem. It (mostly) performs best with NO optional args.

Usage: pff [options] [pattern] [path]
Optional Arguments:
    --help                                  Prints help
    --version                               Prints version

    --include-target                        Include the 'target' directory in the output, if matched
                                            (not included by default)
    -eq                                     Match EXACTLY on 'pattern', faster than (default) regex check 
                                            for exact matching

    --filter <f> [<d> [<s> [<h>]]]          Filter output to just show (f)iles, (d)irectories, (s)ymlinks 
                                            and/or (h)idden files. Providing a 'n' before the parameter 
                                            (e.g. 'nf') hides that item type.

                                            NOTE: The 'f' and 'd' options CANNOT be provided together.
    
    --sort [<asc|desc>]     (default: asc)  Sort output by path in (asc)ending or (desc)ending order. 
                                   
                                            NOTE: Sorting reduces performance and increases memory usage, 
                                            'filtering' results can improve this

    --label [<start|end>] (default: start)  Adds a label, at the start or end of each line separated by a
                                            space, indicating the file properties.

                                            FORMAT : [F|D][R|S|_][R|H]
                                            EXAMPLE: D_R -> dir regular, FSH -> file symlink hidden   

                                            NOTE: Labelling can reduce performance and increases memory usage, 
                                            'filtering' results can improve this
    
    -t   <num>            (default:    {})  Specify the number of threads, MUST BE >= 2
    -fdl <num>            (default:  {})  Specify the maximum 'files + dirs' to traverse before returning
                                            results from each thread

", DEFAULT_NUM_THREADS, DEFAULT_FD_LIMIT);
}

