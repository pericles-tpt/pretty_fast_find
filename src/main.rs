mod walk;
mod find;

#[macro_use]
extern crate lazy_static;


use std::collections::HashSet;
use std::io::Write;
use std::num::ParseIntError;
use std::{collections::HashMap, path::PathBuf};
use std::env;
use find::find;

const HELP_TEXT: &str = "usage: find [TARGET SUBSTRING] [ROOT FIND DIRECTORY]
------- Basic options -------
--help      Print usage and this help message and exit.
------- Find options  -------
-t          (default: 0) Specify the number of threads (must be > 1, otherwise num_threads is set to 0)
-tdl        (default: 256) Specify the minimum number of READDIRs per thread (if not enough dirs are found, this is ignored)";

lazy_static! {
    static ref VALID_COMMAND_OPTIONS: Vec<&'static str> = vec!["-t", "-tdl", "-h", "-s"];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 || args[0].len() == 0 {
        eprintln!("no arguments provided, for a list of commands add the --help argument");
        return;
    }

    let cmd = args[1].as_str();
    let params: Vec<_> = args.iter().skip(2).collect();
    match cmd {
        "find" => {
            if params.len() < 2 {
                eprintln!("insufficient arguments for `find`, expected at least [TARGET SUBSTING] and [ROOT FIND DIRECTORY]");
                return;
            }
            let optional_args: Vec<_> = params.iter().collect();

            let num_args = optional_args.len();

            // Get optional params
            let mut num_threads = 84;
            let mut thread_add_dir_limit = 2048;
            let mut search_hidden = false;
            let mut sorted = false;
            let arg_eval_res = eval_optional_args(optional_args, &mut num_threads, &mut thread_add_dir_limit, &mut search_hidden, &mut sorted);
            if arg_eval_res.is_err() {
                eprintln!("invalid argument provided: {}", arg_eval_res.err().unwrap());
                return;
            }
            
            // Get target substring
            let target_substring = params[num_args - 2];
            
            // Get target directory
            let maybe_target_str = params[num_args - 1];
            let maybe_target_pb = validate_get_pathbuf(maybe_target_str);
            if maybe_target_pb.is_err() {
                eprintln!("invalid target path provided: {}", maybe_target_pb.err().unwrap());
                return;
            }
            let target_pb = maybe_target_pb.unwrap();
            

            let res = find(target_substring.clone(), target_pb, num_threads, thread_add_dir_limit, search_hidden, sorted);
            match res {
                Ok(entries) => {
                    if sorted {
                        let output_str = format!("{}\n", entries.join("\n"));
                        let res = std::io::stdout().write(output_str.as_bytes());
                        if res.is_err() {
                            eprint!("failed to write `find` results to stdout: {:?}", res.err());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error occured while reporting: {}", e);
                }
            }
        }
        "--help" => {
            println!("{}", HELP_TEXT)
        }
        _ => {
            eprintln!("invalid command '{}' provided, must be one of: {}", cmd, VALID_COMMAND_OPTIONS.join(", "));
            return;
        }
    }
    return;
}

fn validate_get_pathbuf(p: &String) -> std::io::Result<PathBuf> {
    let exists = std::fs::exists(p)?;
    if !exists {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("provided path '{}', does not exist", p)));
    }
    return Ok(PathBuf::from(&p));
}


fn eval_optional_args(args: Vec<&&String>, num_threads: &mut usize, thread_add_dir_limit: &mut usize, show_hidden: &mut bool, sorted: &mut bool) -> std::io::Result<()> {  
    let mut i = 0;
    while i < args.len() {
        let before_directory_args = i < args.len() - 2;
        let a = args[i].as_str();
        if before_directory_args && !VALID_COMMAND_OPTIONS.contains(&a) {
            let valid_params: Vec<_> = VALID_COMMAND_OPTIONS.clone().into_iter().collect();
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("invalid parameter '{}' provided for find command, must be one of: {}", a, valid_params.join(", "))));
        }
    
        // NO VALUE OPTIONS
        let mut is_no_val_opt = true;
        match a {
            "-h" => {
                *show_hidden = true;
            }
            "-s" => {
                *sorted = true;

            }
            _ => {is_no_val_opt = false;}
        }
        if is_no_val_opt {
            break;
        }

        // ONE VALUE OPTIONS
        i += 1;
        if i >= args.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("missing additional argument for '{}' flag", a)));
        }
        match a {
            "-t" => {
                let maybe_threads: Result<usize, ParseIntError> = args[i].parse();
                if maybe_threads.is_err() || maybe_threads.clone().unwrap() < 1 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid thread argument, must be a non-negative integer"));
                }
                *num_threads = maybe_threads.unwrap();
            }
            "-tdl" => {
                let maybe_thread_add_dir_limit: Result<usize, ParseIntError> = args[i].parse();
                if maybe_thread_add_dir_limit.is_err() || maybe_thread_add_dir_limit.clone().unwrap() < 1 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid thread add dir limit argument, must be a non-negative integer"));
                }
                *thread_add_dir_limit = maybe_thread_add_dir_limit.unwrap();
            }
            _ => {
                if before_directory_args {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unimplemented parameter: {}, for find command", a)));
                }
            }
        }
        i += 1;
    }
 
    Ok(())    
}