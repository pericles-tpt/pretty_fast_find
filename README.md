# Pretty Fast Find
Pretty Fast Find (`pff`) is an iterative, multithreaded alternative to 'find' that's faster than most alternatives. It provides in-built functionality for filtering, sorting and labelling its output. These in-built features reduce the need for 'piping' to external tools, allowing you to tweak its output without a big performance penalty.

This was originally a command in my [seye_rs](https://github.com/pericles-tpt/seye_rs) project, but once I saw the focus of that project shifting to "find" functionality I decided to separate it into this repo.

## How it works
`pff` does breadth-first, iterative traversals of a list of directories (initially a list containing just the target directory) up to a limit (specified by the `-fdl` parameter). It uses the `rayon` library to multi-thread those traversals up to a thread limit (specified by the `-t` parameter). It also uses the `regex` library for parsing the user provided "pattern" as regex and matching that pattern against file names.

## Example
In this example the results have been filtered to contain just files, sorted in descending order with a property label at the start. The included labels indicate the following:
- FRR -> File Regular Regular
- FSR -> File Symlink Regular
- FRH -> File Regular Hidden

```
 % ./target/release/pff --filter f --sort desc --label start lldb /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/llvm/utils/lldbDataFormatters.py
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/llvm/utils/gn/secondary/lldb/utils/TableGen/lldb_tablegen.gni
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/llvm/utils/gn/secondary/lldb/test/lldb_lit_site_cfg_files.gni
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/llvm/docs/CommandGuide/lldb-tblgen.rst
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/lldb/utils/lui/lldbutil.py
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/lldb/utils/lldb-dotest/lldb-dotest.in
FRR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/lldb/use_lldb_suite_root.py
FSR /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/lldb/tools/lldb-vscode
...
FRH /run/media/pt/gen4_test/llvm-project-llvmorg-20.1.0/lldb/examples/test/.lldb-loggings
...
```
NOTE: The output above is only a sample of the complete command output. Skipped section(s) are denoted with `...`

For more information use the `--help` flag.
## Benchmarks
See `BENCHMARKS.md`

## Faster Alternatives
Most of these options are in an experimental state and/or are missing some of the features of `pff`. However, they run faster, in some or all cases, than `pff`:
- `pvf`: https://mastodon.social/@pervognsen/110739397974530013
- `fdf`: https://github.com/alexcu2718/fdf
