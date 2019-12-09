# Some Rust ports of the Weave parallel benchmarks

## Introduction and basic howto

@mratsim was curious about how Rust's parallelism libraries would fare on the
task-parallel/data-parallel performance front, so I rolled out a quick Rayon
port of some of [Weave's parallel benchmarks](https://github.com/mratsim/weave/tree/master/benchmarks).

You can build & run a benchmark using `cargo run --release --bin <name> <args>`.
For example, to run the dfs microbenchmark with depth 10 and breadth 9, you
would do `cargo run --release --bin dfs 10 9`.

Alternatively, since `cargo run`'s design is not so friendly to performance
analysis tools like perf or GNU time, you may prefer to build everything with
`cargo build --release --all`, then run the generated binaries located in
`./target/release/<name>` yourself as a separate step.

## Idiomatic Rayon constructs for optimal performance

By default, the benchmarks try to replicate the behavior of the Weave benchmark
as closely as possible, even if that means using a Rayon code path that is
well-known to have suboptimal performance and can be easily replaced with a more
performant alternative. To unleash Rayon's full potential, pass the
`--features idiomatic` flag to the above `cargo build` or `cargo run` commands.

For example, to build all benchmarks with idiomatic use of Rayon, you would
do `cargo build --release --features idiomatic --all`.

On my machine, a well-tuned Rayon program has performance that's roughly on par
with the Weave version. This is actually a pretty impressive result for Weave
given the difficult constraints which this library currently operates under
(strict Cilk-like task spawning model without optimized primitives for binary
fork-join or data parallelism, message-passing based implementation,
fine-grained task synchronization, ...).
