# Some Rust ports of the Weave parallel benchmarks

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