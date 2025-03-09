# Pretty Fast Find (pff)
An iterative, multithreaded alternative to 'find' that's faster than most alternatives. This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

WARNING: This isn't a feature complete `find` alternative and most of the functionality has been implemented for the purposes of comparison to existing tools

## How it works
`pff` does breadth-first, iterative traversals of a list of directories up to a limit (specified by the `-tdl` parameter). It uses `rayon` to multi-thread those traversals up to a thread limit (specified by the `-t` parameter).

## Current Performance (benchmarked with `hyperfine`, with `--warmup 250`) 
The following are the results of benchmarking this tool against `fd`, `bfs` and `find`

### System
- Ryzen 9800X3D (8C/16T, 96MB L3 cache)
- 48GB RAM (2x24GB DDR5 6000)
- NM790 1TB Gen4 SSD

### Small Number of Results (58 results)
Description: Looking for file names matching "Document" in the shallow "Documents" directory

Winner: `pff` is 15.31% faster than 2nd place, `fd`
#### find
```
Benchmark 1: find /run/media/pt/gen4_test/pt/Documents -name '*Document*'
  Time (mean ± σ):     317.7 ms ±   3.8 ms    [User: 113.8 ms, System: 203.1 ms]
  Range (min … max):   303.1 ms … 329.2 ms    1000 runs
```
#### bfs
```
Benchmark 1: bfs -name '*Document*' -nocolor /run/media/pt/gen4_test/pt/Documents
  Time (mean ± σ):      69.8 ms ±   3.3 ms    [User: 81.7 ms, System: 245.5 ms]
  Range (min … max):    64.6 ms …  81.9 ms    1000 runs
```
#### fd
```
Benchmark 1: fd -I -H --color never Document /run/media/pt/gen4_test/pt/Documents
  Time (mean ± σ):      36.9 ms ±   1.5 ms    [User: 162.6 ms, System: 324.8 ms]
  Range (min … max):    33.4 ms …  42.9 ms    1000 runs
```
#### pff
```
Benchmark 1: ./target/release/pff find -t 84 -tdl 2048 -h Document /run/media/pt/gen4_test/pt/Documents
  Time (mean ± σ):      32.0 ms ±   1.1 ms    [User: 90.6 ms, System: 283.9 ms]
  Range (min … max):    30.1 ms …  41.9 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```
### Large Number of Results (228254 results)
Description: Looking for file names matching "js" in the deep "pt" directory

Winner: `pff` is 15.95% faster than 2nd place, `fd`
#### find (NOTE: Reduced runs here because I couldn't be bothered waiting for 1000 x 1.2s runs to finish)
```
Benchmark 1: find /run/media/pt/gen4_test/pt -name '*js*'
  Time (mean ± σ):      1.253 s ±  0.007 s    [User: 0.465 s, System: 0.785 s]
  Range (min … max):    1.240 s …  1.266 s    10 runs
```
#### bfs
```
Benchmark 1: bfs -name '*js*' -nocolor /run/media/pt/gen4_test/pt
  Time (mean ± σ):     307.3 ms ±  15.0 ms    [User: 341.5 ms, System: 980.7 ms]
  Range (min … max):   283.9 ms … 398.1 ms    1000 runs
```
#### fd
```
Benchmark 1: fd -I -H --color never js /run/media/pt/gen4_test/pt
  Time (mean ± σ):     139.9 ms ±   8.8 ms    [User: 672.2 ms, System: 1251.4 ms]
  Range (min … max):   127.9 ms … 176.2 ms    1000 runs
```
#### pff
```
Benchmark 1: ./target/release/pff find -t 84 -tdl 2048 -h js /run/media/pt/gen4_test/pt
  Time (mean ± σ):     117.6 ms ±   2.0 ms    [User: 356.1 ms, System: 1115.7 ms]
  Range (min … max):   112.0 ms … 133.4 ms    1000 runs
```

### Sorting
The alternative programs don't have in-built sort functionality as far as I know. You can pipe their output to the unix `sort` command, but this increases the time taken to ~2s for the "Large" test.

`pff` has an in-built `-s` flag that sorts the output with a comparatively minor time penalty, the average execution times from enabling this option are:
- large: 117.6ms -> 177.6ms (33.78% slower)
- small: 32.7ms  -> 34.7ms  (5.77% slower)

### CPU Usage
Although I didn't quantitatively measure it, `pff` appeared to have lower CPU usage than `fd`. On the other hand the `find`/`bfs` commands had lower CPU usage than `pff` but also had significantly worse performance.

## Planned Features
- Add an option to specify a memory usage limit
