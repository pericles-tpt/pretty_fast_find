# Pretty Fast Find
Pretty Fast Find (`pff`) is an iterative, multithreaded alternative to 'find' that's faster than most alternatives. This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

WARNING: This isn't a feature complete `find` alternative and most of its functionality has been implemented for the purposes of comparison to existing tools

## How it works
`pff` does breadth-first, iterative traversals of a list of directories (initially a list containing just the target directory) up to a limit (specified by the `-fdl` parameter). It uses `rayon` to multi-thread those traversals up to a thread limit (specified by the `-t` parameter).

## Current Performance (benchmarked with `hyperfine`, with `--warmup 250`) 
The following are the results of benchmarking this tool against `fd` (v10.1.0), `bfs` (v4.0.4) and `find` (v4.10.0). The sample folder used for testing is [llvm-project v20.1.0 source code](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0), it has ~150k files and ~14.5k folders (according to `find`).

### System 1
- OS : Fedora Workstation
- CPU: Ryzen 9800X3D (8C/16T, 96MB L3 cache)
- RAM: 48GB (2x24GB DDR5 6000)
- SSD: NM790 1TB Gen4 SSD

#### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

##### find (9.49x slower)
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):      93.9 ms ±   2.8 ms    [User: 41.8 ms, System: 51.8 ms]
  Range (min … max):    89.0 ms … 106.3 ms    1000 runs
```
##### bfs (3.34x slower)
```
Benchmark 1: bfs -name '*clang*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      33.1 ms ±   1.4 ms    [User: 36.5 ms, System: 63.7 ms]
  Range (min … max):    30.7 ms …  38.3 ms    1000 runs
```
##### fd (1.35x slower)
```
Benchmark 1: fd -I -H --color never -s clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      13.4 ms ±   0.8 ms    [User: 62.9 ms, System: 76.4 ms]
  Range (min … max):    11.4 ms …  19.5 ms    1000 runs
```
##### pff
```
Benchmark 1: ./target/release/pff clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):       9.9 ms ±   0.3 ms    [User: 37.3 ms, System: 66.4 ms]
  Range (min … max):     9.0 ms …  11.5 ms    1000 runs
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (7.72x slower)
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):      93.5 ms ±   2.5 ms    [User: 41.7 ms, System: 51.5 ms]
  Range (min … max):    88.9 ms … 106.0 ms    1000 runs
```
##### bfs (2.70x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      32.7 ms ±   1.3 ms    [User: 36.0 ms, System: 63.7 ms]
  Range (min … max):    30.7 ms …  38.1 ms    1000 runs
```
##### fd (1.32x slower)
```
Benchmark 1: fd -I -H --color never -s s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      15.9 ms ±   0.8 ms    [User: 66.0 ms, System: 76.3 ms]
  Range (min … max):    13.9 ms …  22.2 ms    1000 runs
```
##### pff
```
Benchmark 1: ./target/release/pff s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      12.1 ms ±   0.4 ms    [User: 52.9 ms, System: 71.5 ms]
  Range (min … max):    11.0 ms …  14.0 ms    1000 runs
```

### System 2 (HP 800 G5 DM)
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
Benchmark 1: ./target/release/pff clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      34.1 ms ±   0.8 ms    [User: 93.7 ms, System: 93.1 ms]
  Range (min … max):    32.1 ms …  40.1 ms    1000 runs
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (6.43x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):     266.9 ms ±   1.4 ms    [User: 150.0 ms, System: 116.3 ms]
  Range (min … max):   263.7 ms … 276.0 ms    1000 runs
```
##### bfs (1.82x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      75.4 ms ±   0.9 ms    [User: 94.7 ms, System: 98.3 ms]
  Range (min … max):    74.3 ms …  86.0 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### fd (1.23x slower)
```
Benchmark 1: fd -I -H --color never -s s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      51.1 ms ±   3.3 ms    [User: 168.4 ms, System: 119.7 ms]
  Range (min … max):    45.8 ms …  90.6 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      41.5 ms ±   1.5 ms    [User: 122.6 ms, System: 100.8 ms]
  Range (min … max):    39.1 ms …  49.9 ms    1000 runs
```

### Sorting
The alternative programs don't have in-built sort functionality as far as I know. You can pipe their output to the unix `sort` command, but this increases the time taken for each by ~630ms for the "Large" test.

`pff` has an in-built `-s` flag that sorts the output with a relatively minor time penalty, the average execution times from enabling this option (on System 1) are:
- small: 10.0ms -> 10.1ms (1.00% slower)
- large: 11.6ms -> 29.2ms (151.72% slower)

### CPU Usage
Although I didn't quantitatively measure it, `pff` appeared to have lower CPU usage than `fd`. On the other hand the `find`/`bfs` commands had lower CPU usage than `pff` but also had significantly worse performance.

## Planned Features
- Add an option to specify a memory usage limit
