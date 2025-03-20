## Benchmarks (using `hyperfine`, with `--warmup 250`) 
The following are the results of benchmarking this tool against `fd` (v10.1.0), `bfs` (v4.0.4) and `find` (v4.10.0). The sample folder used for testing is [llvm-project v20.1.0 source code](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0), it has ~150k files and ~14.5k folders (according to `find`).

### System 1
- OS : Fedora Workstation
- CPU: Ryzen 9800X3D (8C/16T, 96MB L3 cache)
- RAM: 48GB (2x24GB DDR5 6000)
- SSD: NM790 1TB Gen4 SSD

#### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

##### find (10.10x slower)
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):      89.9 ms ±   0.8 ms    [User: 42.5 ms, System: 47.1 ms]
  Range (min … max):    87.9 ms …  95.3 ms    1000 runs
```
##### bfs (3.65x slower)
```
Benchmark 1: bfs -name '*clang*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      32.5 ms ±   1.4 ms    [User: 36.2 ms, System: 60.0 ms]
  Range (min … max):    30.5 ms …  38.6 ms    1000 runs
```
##### fd (1.48x slower)
```
Benchmark 1: fd -I -H --color never -s clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      13.2 ms ±   0.8 ms    [User: 63.5 ms, System: 75.0 ms]
  Range (min … max):    11.1 ms …  16.3 ms    1000 runs
```
##### pff
```
Benchmark 1: ./target/release/pff  clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):       8.9 ms ±   0.2 ms    [User: 33.3 ms, System: 65.4 ms]
  Range (min … max):     8.0 ms …   9.6 ms    1000 runs
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (8.73x slower)
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):      89.9 ms ±   0.8 ms    [User: 42.3 ms, System: 47.4 ms]
  Range (min … max):    87.8 ms …  95.9 ms    1000 runs
```
##### bfs (3.14x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      32.3 ms ±   1.3 ms    [User: 36.0 ms, System: 60.5 ms]
  Range (min … max):    30.6 ms …  37.3 ms    1000 runs
```
##### fd (1.51x slower)
```
Benchmark 1: fd -I -H --color never -s s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      15.6 ms ±   0.8 ms    [User: 65.0 ms, System: 73.1 ms]
  Range (min … max):    13.5 ms …  18.3 ms    1000 runs
```
##### pff
```
Benchmark 1: ./target/release/pff s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      10.3 ms ±   0.3 ms    [User: 48.3 ms, System: 68.1 ms]
  Range (min … max):     9.5 ms …  11.5 ms    1000 runs
```

### System 2 (HP 800 G5 DM)
- OS : Fedora Server
- CPU: Intel i5 9500T (6C/12T, 9MB L3 cache)
- RAM: 32GB (2x16GB DDR4 SODIMM 3200)
- SSD: 1TB SATA SSD (HP S750)

#### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

##### find (8.42x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):     265.3 ms ±   1.5 ms    [User: 150.7 ms, System: 114.0 ms]
  Range (min … max):   262.3 ms … 273.5 ms    1000 runs
```
##### bfs (2.32x slower)
```
Benchmark 1: bfs -name '*clang*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      73.2 ms ±   1.2 ms    [User: 93.1 ms, System: 96.1 ms]
  Range (min … max):    72.1 ms …  88.6 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### fd  (1.27x slower)
```
Benchmark 1: fd -I -H --color never -s clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      40.0 ms ±   2.6 ms    [User: 127.9 ms, System: 104.0 ms]
  Range (min … max):    36.1 ms …  73.7 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      31.5 ms ±   1.0 ms    [User: 83.6 ms, System: 94.3 ms]
  Range (min … max):    29.8 ms …  43.1 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (6.98x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):     266.0 ms ±   1.5 ms    [User: 150.7 ms, System: 114.7 ms]
  Range (min … max):   262.8 ms … 276.7 ms    1000 runs
```
##### bfs (1.98x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      75.4 ms ±   0.9 ms    [User: 94.3 ms, System: 97.8 ms]
  Range (min … max):    74.3 ms …  88.7 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### fd (1.34x slower)
```
Benchmark 1: fd -I -H --color never -s s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      50.9 ms ±   3.1 ms    [User: 169.2 ms, System: 118.1 ms]
  Range (min … max):    45.9 ms …  95.5 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      38.1 ms ±   0.9 ms    [User: 111.2 ms, System: 97.8 ms]
  Range (min … max):    36.0 ms …  48.4 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```

### Memory Usage
I haven't properly measured memory usage yet, but from the peak values I observed on system monitor (again not scientific) it appears to be:

| Program | Memory Usage | Memory Usage Multiplier |
| ------- | ------------ | ----------------------- |
| pff     | 14.5MB       | 1.00x                   |
| fd      | 14.3MB       | 0.98x                   |
| bfs     |  7.7MB       | 0.53x                   |
| find    |  0.7MB       | 0.05x                   |

Comparing these (rough) results to the benchmarks above, `pff`'s performance improvement appears to exceed the increase in memory consumption (as a percentage) in all cases except for `find`.

NOTE: These results only apply for unsorted runs, where each thread can print its results and then immediately discard them. However, when sorting results `pff` needs to store every result until the end of the program in order to sort and print them out all out at once. As a result, `pff` has been observed to use as much as 60MB of memory when sorting in the "large" benchmark (albeit this usage occurs for < 20ms).

### Sorting
The alternative programs don't have in-built sort functionality as far as I know. You can pipe their output to the unix `sort` command, but this increases the time taken for each program by ~645ms for the "large" test.

`pff` has an in-built `--sort` flag that sorts the output with a less severe time penalty, the average execution times from enabling this option (on System 1) are:

| Benchmark Type | Unsorted | Sorted | Time Penalty |
| -------------- | -------- | ------ | ------------ |
| small          | 8.9ms    | 9.5ms  | 1.07x        |
| large          | 10.3ms   | 22.6ms | 2.19x        |

### CPU Usage
Although I didn't quantitatively measure it, `pff` appeared to have lower CPU usage than `fd`. On the other hand the `find`/`bfs` commands had lower CPU usage than `pff` but also had significantly worse performance.