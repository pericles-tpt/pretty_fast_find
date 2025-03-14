## Benchmarks [OUTDATED, PRIOR COMMIT IMPACTED PERFORMANCE] (using `hyperfine`, with `--warmup 250`) 
The following are the results of benchmarking this tool against `fd` (v10.1.0), `bfs` (v4.0.4) and `find` (v4.10.0). The sample folder used for testing is [llvm-project v20.1.0 source code](https://github.com/llvm/llvm-project/releases/tag/llvmorg-20.1.0), it has ~150k files and ~14.5k folders (according to `find`).

### System 1
- OS : Fedora Workstation
- CPU: Ryzen 9800X3D (8C/16T, 96MB L3 cache)
- RAM: 48GB (2x24GB DDR5 6000)
- SSD: NM790 1TB Gen4 SSD

#### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

##### find (10.36x slower)
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):      91.2 ms ±   1.4 ms    [User: 42.5 ms, System: 48.4 ms]
  Range (min … max):    88.6 ms … 103.0 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### bfs (3.77x slower)
```
Benchmark 1: bfs -name '*clang*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      33.2 ms ±   1.3 ms    [User: 36.9 ms, System: 62.6 ms]
  Range (min … max):    30.9 ms …  38.0 ms    1000 runs

```
##### fd (1.52x slower)
```
Benchmark 1: fd -I -H --color never -s clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      13.4 ms ±   0.8 ms    [User: 63.2 ms, System: 75.7 ms]
  Range (min … max):    11.6 ms …  16.7 ms    1000 runs
```
##### pff
```
Benchmark 1: ./target/release/pff clang /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):       8.8 ms ±   0.2 ms    [User: 32.9 ms, System: 65.8 ms]
  Range (min … max):     8.1 ms …   9.6 ms    1000 runs
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (8.87x slower)
```
Benchmark 1: find /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):      91.4 ms ±   1.5 ms    [User: 42.1 ms, System: 49.0 ms]
  Range (min … max):    88.6 ms … 102.2 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### bfs (3.22x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      33.2 ms ±   1.3 ms    [User: 36.6 ms, System: 63.4 ms]
  Range (min … max):    31.1 ms …  38.3 ms    1000 runs
```
##### fd (1.55x slower)
```
Benchmark 1: fd -I -H --color never -s s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      16.0 ms ±   0.8 ms    [User: 65.9 ms, System: 75.5 ms]
  Range (min … max):    13.9 ms …  18.7 ms    1000 runs
```
##### pff
```
Benchmark 1: ./target/release/pff s /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      10.3 ms ±   0.3 ms    [User: 48.4 ms, System: 68.4 ms]
  Range (min … max):     9.5 ms …  11.3 ms    1000 runs
```

### System 2 (HP 800 G5 DM)
- OS : Fedora Server
- CPU: Intel i5 9500T (6C/12T, 9MB L3 cache)
- RAM: 32GB (2x16GB DDR4 SODIMM 3200)
- SSD: 1TB SATA SSD (HP S750)

#### Small Number of Results (543 results)
Description: Looking for file names containing "clang"

##### find (8.48x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*clang*'
  Time (mean ± σ):     265.4 ms ±   1.6 ms    [User: 149.7 ms, System: 115.1 ms]
  Range (min … max):   262.4 ms … 282.1 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### bfs (2.34x slower)
```
Benchmark 1: bfs -name '*clang*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      73.2 ms ±   1.3 ms    [User: 93.6 ms, System: 96.2 ms]
  Range (min … max):    72.0 ms …  89.3 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### fd  (1.29x slower)
```
Benchmark 1: fd -I -H --color never -s clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      40.3 ms ±   2.7 ms    [User: 128.7 ms, System: 104.6 ms]
  Range (min … max):    35.6 ms …  83.5 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff clang /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      31.3 ms ±   1.0 ms    [User: 83.9 ms, System: 92.9 ms]
  Range (min … max):    29.5 ms …  43.1 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
#### Large Number of Results (107410 results)
Description: Looking for file names containing "s"

##### find (7.26x slower)
```
Benchmark 1: find /home/pt/pff_test/llvm-project-llvmorg-20.1.0 -name '*s*'
  Time (mean ± σ):     267.1 ms ±   1.8 ms    [User: 150.5 ms, System: 116.0 ms]
  Range (min … max):   263.3 ms … 283.1 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### bfs (2.06x slower)
```
Benchmark 1: bfs -name '*s*' -nocolor /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      75.7 ms ±   0.8 ms    [User: 95.3 ms, System: 97.9 ms]
  Range (min … max):    74.6 ms …  84.1 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### fd (1.40x slower)
```
Benchmark 1: fd -I -H --color never -s s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      51.5 ms ±   2.6 ms    [User: 170.0 ms, System: 120.6 ms]
  Range (min … max):    46.2 ms …  74.4 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
##### pff
```
Benchmark 1: ./target/release/pff s /home/pt/pff_test/llvm-project-llvmorg-20.1.0
  Time (mean ± σ):      36.8 ms ±   0.9 ms    [User: 110.8 ms, System: 96.7 ms]
  Range (min … max):    34.9 ms …  43.9 ms    1000 runs
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

`pff` has an in-built `-s` flag that sorts the output with a less severe time penalty, the average execution times from enabling this option (on System 1) are:

| Benchmark Type | Unsorted | Sorted | Time Penalty |
| -------------- | -------- | ------ | ------------ |
| small          | 8.8ms    | 9.1ms  | 1.03x        |
| large          | 10.3ms   | 21.1ms | 2.05x        |

### CPU Usage
Although I didn't quantitatively measure it, `pff` appeared to have lower CPU usage than `fd`. On the other hand the `find`/`bfs` commands had lower CPU usage than `pff` but also had significantly worse performance.