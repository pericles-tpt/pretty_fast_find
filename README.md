# Pretty Fast Find (pff)
An iterative, multithreaded alternative to 'find' that's faster than most alternatives. This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

WARNING: This isn't a feature complete `find` alternative and most of its functionality has been implemented for the purposes of comparison to existing tools

## How it works
`pff` does breadth-first, iterative traversals of a list of directories up to a limit (specified by the `-tdl` parameter). It uses `rayon` to multi-thread those traversals up to a thread limit (specified by the `-t` parameter).

## Current Performance (benchmarked with `hyperfine`, with `--warmup 250`) 
The following are the results of benchmarking this tool against `fd` (v10.1.0), `bfs` (v4.0.4) and `find` (v4.10.0). The sample folder used for testing is [llvm-project v20.1.0 source code](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0), it has ~150k files and ~14.5k folders (according to `find`).

### System
- Ryzen 9800X3D (8C/16T, 96MB L3 cache)
- 48GB RAM (2x24GB DDR5 6000)
- NM790 1TB Gen4 SSD

### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

Winner: `pff` is 25.37% faster than 2nd place, `fd`
#### find
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):      93.9 ms ±   2.8 ms    [User: 41.8 ms, System: 51.8 ms]
  Range (min … max):    89.0 ms … 106.3 ms    1000 runs
```
#### bfs
```
Benchmark 1: bfs -name '*clang*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      33.1 ms ±   1.4 ms    [User: 36.5 ms, System: 63.7 ms]
  Range (min … max):    30.7 ms …  38.3 ms    1000 runs
```
#### fd
```
Benchmark 1: fd -I -H --color never -s clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      13.4 ms ±   0.8 ms    [User: 62.9 ms, System: 76.4 ms]
  Range (min … max):    11.4 ms …  19.5 ms    1000 runs
```
#### pff
```
Benchmark 1: ./target/release/pff find -t 84 -tdl 2048 -h clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      10.0 ms ±   0.4 ms    [User: 38.6 ms, System: 66.3 ms]
  Range (min … max):     9.2 ms …  17.8 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

Winner: `pff` is 27.04% faster than 2nd place, `fd`
#### find
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):      93.5 ms ±   2.5 ms    [User: 41.7 ms, System: 51.5 ms]
  Range (min … max):    88.9 ms … 106.0 ms    1000 runs
```
#### bfs
```
Benchmark 1: bfs -name '*s*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      32.7 ms ±   1.3 ms    [User: 36.0 ms, System: 63.7 ms]
  Range (min … max):    30.7 ms …  38.1 ms    1000 runs
```
#### fd
```
Benchmark 1: fd -I -H --color never -s s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      15.9 ms ±   0.8 ms    [User: 66.0 ms, System: 76.3 ms]
  Range (min … max):    13.9 ms …  22.2 ms    1000 runs
```
#### pff
```
Benchmark 1: ./target/release/pff find -t 84 -tdl 2048 -h s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      11.6 ms ±   0.5 ms    [User: 49.6 ms, System: 70.8 ms]
  Range (min … max):    10.6 ms …  19.3 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```

### Sorting
The alternative programs don't have in-built sort functionality as far as I know. You can pipe their output to the unix `sort` command, but this increases the time taken for each to ~630ms for the "Large" test.

`pff` has an in-built `-s` flag that sorts the output with a relatively minor time penalty, the average execution times from enabling this option are:
- large: 11.6ms -> 29.2ms (151.72% slower)
- small: 10.0ms -> 10.0ms (1.00% slower)

### CPU Usage
Although I didn't quantitatively measure it, `pff` appeared to have lower CPU usage than `fd`. On the other hand the `find`/`bfs` commands had lower CPU usage than `pff` but also had significantly worse performance.

## Planned Features
- Add an option to specify a memory usage limit
