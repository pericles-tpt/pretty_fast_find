# Pretty Fast Find
Pretty Fast Find (`pff`) is an iterative, multithreaded alternative to 'find' that's faster than most alternatives. This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

WARNING: This isn't a feature complete `find` alternative and most of its functionality has been implemented for the purposes of comparison to existing tools

## How it works
`pff` does breadth-first, iterative traversals of a list of directories (initially a list containing just the target directory) up to a limit (specified by the `-fdl` parameter). It uses `rayon` to multi-thread those traversals up to a thread limit (specified by the `-t` parameter).

## Benchmarks
See `BENCHMARKS.md`

## Planned Features
- Add an option to specify a memory usage limit
