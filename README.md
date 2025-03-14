# Pretty Fast Find
Pretty Fast Find (`pff`) is an iterative, multithreaded alternative to 'find' that's faster than most alternatives. It provides in-built functionality for filtering, sorting and labelling its output. These in-built features reduce the need for 'piping' to external tools, allowing you to tweak its output without a big performance penalty.

This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

## How it works
`pff` does breadth-first, iterative traversals of a list of directories (initially a list containing just the target directory) up to a limit (specified by the `-fdl` parameter). It uses the excellent `rayon` library to multi-thread those traversals up to a thread limit (specified by the `-t` parameter). It also uses the `regex` library for parsing the user provided "pattern" as regex and matching that pattern against file names.

## Benchmarks
See `BENCHMARKS.md`

## Planned Features
- Add an option to specify a memory usage limit
