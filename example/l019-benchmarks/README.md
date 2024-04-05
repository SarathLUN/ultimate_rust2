# Lesson 19: Benchmarks

1. Benchmarking with Criterion:

- Rust's built-in benchmarking is not ideal for production use.
- Criterion is the preferred tool for benchmarking Rust code.
- Criterion required adding it as development dependency in `Cargo.toml`.
- Benchmarks are defined in separate Rust files within a `benches` directory.

2. Benchmark Function Structure:

- Benchmark functions accept a mutable reference to a `Criterion` struct.
- the `.bench_function` method is used to define a benchmark.
- closures are used to define the code to be benchmarked.
- the `black_box` macro prevents compiler optimizations from affecting the benchmark.

3. Running benchmarks:

- use `cargo bench` command to run benchmarks.
- benchmark involve warm up period and a measurement period.
- the output show statistics for each benchmark run.

4. Interpreting Result:

- the bold statistic in the output represents the median measurement.
- outliers can indicate external factors affecting benchmark results.
- benchmark should be run on dedicated hardware with minimal background noise.

5. Importance of benchmark:

- benchmark help identify bottlenecks in your code.
- don't prematurely optimize, start with idiomatic code and benchmark later.
- benchmark result can vary depending on hardware, OS, Rust version, etc...

6. HTML report:

- Criterion can generate HTML report for visualizing benchmark result.
- report include graphs and comparisons between benchmark runs.

### to test this lab:

- there are 3 benchmark approaches in file: `src/lib.rs`
- un-comment each benchmark 1 by 1 and run `cargo bench`
