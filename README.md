# Pretty Fast Find
Pretty Fast Find (`pff`) is an iterative, multithreaded alternative to 'find' that's faster than most alternatives. This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

WARNING: This isn't a feature complete `find` alternative and most of its functionality has been implemented for the purposes of comparison to existing tools

## How it works
`pff` does breadth-first, iterative traversals of a list of directories (initially a list containing just the target directory) up to a limit (specified by the `-tdl` parameter). It uses `rayon` to multi-thread those traversals up to a thread limit (specified by the `-t` parameter).

## Current Performance (benchmarked with `hyperfine`, with `--warmup 250`) 
The following are the results of benchmarking this tool against `fd` (v10.1.0), `bfs` (v4.0.4) and `find` (v4.10.0). The sample folder used for testing is [llvm-project v20.1.0 source code](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0), it has ~150k files and ~14.5k folders (according to `find`).

### System
- OS : Fedora Server
- CPU: Intel i5 9500T (6C/12T, 9MB L3 cache)
- RAM: 32GB (2x16GB DDR4 SODIMM 3200)
- SSD: 1TB SATA SSD (HP S750)

#### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

##### find (7.79x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):     265.6 ms ±   1.4 ms    [User: 150.0 ms, System: 115.0 ms]
  Range (min … max):   263.2 ms … 274.8 ms    1000 runs
```
##### bfs (2.14x slower)
```
Benchmark 1: bfs -name '*clang*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      73.1 ms ±   0.8 ms    [User: 92.8 ms, System: 97.2 ms]
  Range (min … max):    72.3 ms …  88.4 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options
```
##### fd  (1.17x slower)
```
Benchmark 1: fd -I -H --color never -s clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      40.0 ms ±   2.1 ms    [User: 127.2 ms, System: 104.4 ms]
  Range (min … max):    36.0 ms …  68.7 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff find -h clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      34.1 ms ±   0.8 ms    [User: 94.7 ms, System: 94.1 ms]
  Range (min … max):    32.2 ms …  36.8 ms    1000 runs
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (6.94x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):     266.9 ms ±   1.4 ms    [User: 150.0 ms, System: 116.3 ms]
  Range (min … max):   263.7 ms … 276.0 ms    1000 runs
```
##### bfs (1.96x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      75.4 ms ±   0.9 ms    [User: 94.7 ms, System: 98.3 ms]
  Range (min … max):    74.3 ms …  86.0 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### fd (1.33x slower)
```
Benchmark 1: fd -I -H --color never -s s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      51.1 ms ±   3.3 ms    [User: 168.4 ms, System: 119.7 ms]
  Range (min … max):    45.8 ms …  90.6 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff find -h s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      38.5 ms ±   1.6 ms    [User: 115.0 ms, System: 98.1 ms]
  Range (min … max):    36.1 ms …  47.2 ms    1000 runs
```

### Performance Characteristics
`pff` has been observed to perform worse than alternatives in a benchmark scenario, when running on a heavily CPU throttled and memory bandwidth limited machine. This was observed on an Acer B115, which is a passively cooled, pentium n3530, single channel memory laptop.

That system's results were not included as single channel, severely thermally constrained computer are uncommon and less representative than the results above.

### CPU Usage
Although I didn't quantitatively measure it, `pff` appeared to have lower CPU usage than `fd`. On the other hand the `find`/`bfs` commands had lower CPU usage than `pff` but also had significantly worse performance.

## Planned Features
- Add an option to specify a memory usage limit
