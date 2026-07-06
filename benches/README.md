# Benchmark suites

The benchmarks are separated by the behavior they measure:

- `collection` covers collecting owned inline and heap strings, including
  pre-reserved and shared first items.
- `mutation` covers reserve and append paths across inline, unique, shared, and
  static storage.
- `heap_layout` covers construction, clone/drop, and traversal of large string
  vectors around the 12-byte and 16-byte representation boundaries.

Use the same target directory across the baseline and experiment worktrees so
Criterion can compare against saved samples. For example, to record and compare
the collection suite quickly:

```sh
CARGO_TARGET_DIR=/private/tmp/lean-string-perf-target \
  cargo bench --bench collection -- --quick --save-baseline main

CARGO_TARGET_DIR=/private/tmp/lean-string-perf-target \
  cargo bench --bench collection -- --quick --baseline main
```

Drop `--quick` for a full 50-sample run. Substitute `mutation` or `heap_layout`
for the other suites. The heap-layout suite constructs large vectors and takes
materially longer than the public-API suites.

Record the exact command, machine, toolchain, and raw Criterion output for every
scoreboard row. Do not compare quick samples with full samples.

## CodSpeed

The `CodSpeed` workflow runs every benchmark with the simulation and memory
instruments on pushes to `main` and on pull requests. To verify the integration
locally after installing `cargo-codspeed`:

```sh
cargo codspeed build --measurement-mode simulation --measurement-mode memory
cargo codspeed run --measurement-mode simulation
cargo codspeed run --measurement-mode memory
```

Ordinary `cargo bench` commands still use Criterion's local statistical
measurements. CodSpeed must be connected to the public GitHub repository before
the workflow can upload results, and a run on `main` is required before pull
requests have a comparison baseline. CodSpeed's memory instrument records
allocations for the heap-layout workloads; local Criterion runs measure time
only.
