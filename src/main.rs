use std::env;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;
use std::path::PathBuf;

mod find;
mod walk;

use find::find;

const DEFAULT_NUM_THREADS: usize = 84;
const DEFAULT_FD_LIMIT: usize = 2048;

#[derive(Clone)]
struct Config {
    num_threads: usize,
    file_dir_limit: usize,
    show_files: bool,
    show_dirs: bool,
    show_symlinks: bool,
    show_hidden: bool,
    sorted: bool,
    sort_asc: bool,
    equality_match: bool,
}

fn main() {
    let mut cfg = Config {
        num_threads:    DEFAULT_NUM_THREADS,
        file_dir_limit: DEFAULT_FD_LIMIT,
        show_files:     true,
        show_dirs:      true,
        show_symlinks:  true,
        show_hidden:    true,
        sorted:         false,
        sort_asc:       true,
        equality_match: false,
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
            eprintln!("{}", e);
            return;
        }
    }

    match find(target, root, &cfg) {
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
            eprintln!("{}", e);
        }
    }
}


fn eval_args(args: &Vec<String>, config: &mut Config) -> std::io::Result<(String, PathBuf)> {
    // Length Checks / Help
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
    let maybe_root = &args[args.len() - 1];
    if !std::fs::exists(maybe_root)? {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("provided path '{}', does not exist", maybe_root)));
    }
    let root_pb = PathBuf::from(maybe_root);
    let has_optional_args = args.len() > 2;
    if !has_optional_args {
        return Ok((target, root_pb));
    }

    // Optional Args
    let mut i = 0;
    let first_non_optional_arg_idx = args.len() - 2;
    let valid_command_options = vec!["--help", "--version", "-eq", "-hf", "-hd", "-hsl", "-hh", "-sa", "-sd", "-t", "-fdl"];
    while i < first_non_optional_arg_idx {
        let curr = args[i].as_str();
        if !valid_command_options.contains(&curr) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid argument '{}', must be one of: {}", curr, valid_command_options.join(", "))));
        }
    
        // Toggle Args
        let mut is_valid_opt = true;
        match curr {
            "-eq" => {
                config.equality_match = true;
            }
            "-hf" => {
                config.show_files = false;
            }
            "-hd" => {
                config.show_dirs = false;
            }
            "-hsl" => {
                config.show_symlinks = false;
            }
            "-hh" => {
                config.show_hidden = false;
            }
            "-sa" => {
                config.sorted = true;
            }
            "-sd" => {
                config.sorted = true;
                config.sort_asc = false;
            }
            _ => { is_valid_opt = false; }
        }
        i += 1;
        if is_valid_opt {
            continue;
        }
        
        // Single-value Args
        if i + 1 >= args.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("missing additional argument for '{}' flag", curr)));
        }
        let next = args[i].as_str();
        match curr {
            "-t" => {
                let maybe_num_threads: Result<usize, ParseIntError> = next.parse();
                if maybe_num_threads.is_err() || maybe_num_threads.clone().unwrap() < 1 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid `-t` argument, must be a non-negative integer"));
                }
                config.num_threads = maybe_num_threads.unwrap();
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

    if !config.show_files && !config.show_dirs {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid combination of '-hd' and '-fd', can only accept one at a time"));
    }
 
    Ok((target, root_pb))    
}

fn print_help_text() {
    println!("Pretty Fast Find, finds items in your filesystem. It (mostly) performs best with NO optional args.

Usage: pff [options] [pattern] [path]
Optional Arguments:
    --help        Prints help
    --version     Prints version

    -eq           Match EXACTLY on 'pattern', faster than (default) regex check for exact matching
    -hf           Hides files (CANNOT be used with -hd flag)
    -hd           Hides directories (CANNOT be used with -hf flag)
    -hsl          Hides symlinks
    -hh           Hides hidden dirs/files (i.e. entries where name starts with '.')
    -sa           Sort output by path in ascending order
    -sd           Sort output by path in descending order

    -t   <num>    (default:    {}) Specify the number of threads, MUST BE >= 2
    -fdl <num>    (default:  {}) Specify the maximum 'files + dirs' to traverse before returning
                                 results from each thread

NOTE: Sorting reduces performance and increases memory usage, 'hiding' results can improve this", DEFAULT_NUM_THREADS, DEFAULT_FD_LIMIT);
}
