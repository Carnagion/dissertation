# Project Structure

There are two parts to this project:
1. The mathematical program, found under the `cplex/` directory
2. The branch-and-bound program, found under the `runseq/` directory

The mathematical program consists of a single OPL source file (`cplex/runseq.mod`) and data file (`cplex/runseq.dat`), along with driver code (`cplex/test.mod`) for running benchmarks.

The branch-and-bound program is structured as a [`cargo` workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) with multiple components under it, each one an individual Rust library:
- `runseq-instance/` contains core data types and definitions used by all other Rust code in the project.
- `runseq-branch-bound/` contains the actual implementation of the branch-and-bound algorithm.
- `runseq-data/` contains helpers for parsing problem instances and datasets into the types defined in `runseq-instance/`.
- `runseq-vis/` contains the sequence visualiser implementation.

The directory for each component contains an `src/` sub-directory with the source code of that component.
The top-level `runseq/` directory itself is a Rust library that depends on these smaller libraries.

Additionally, there are also `benches/` and `examples/` directories under `runseq/` containing branch-and-bound benchmarks and examples that can be run.
This is explained further in [Building and Running Code](#building-and-running-code).

# Installation and Setup

For the mathematical program, install the latest version of [IBM ILOG CPLEX Optimisation Studio](https://www.ibm.com/products/ilog-cplex-optimization-studio).
Note that this project was built and tested on CPLEX version 22.1.1.
Open CPLEX to verify that it was installed successfully.

For the branch-and-bound program, install the latest version of [Rust](https://www.rust-lang.org/tools/install) via `rustup`.
Note that this project was built and tested on `rustup` v1.26.0 and `rustc` v1.77.0.
Future minor versions (`1.x.y`) should work due to backwards compatibility, but earlier versions may not.
Run `rustup --version`, `rustc --version`, and `cargo --version` to verify that it was installed successfully.

# Dependencies

The mathematical program does not use any dependencies.

The branch-and-bound program uses a number of dependencies, all of which are automatically downloaded and built by `cargo` when compiling the project - no manual installation of dependencies is necessary.
A list of all direct dependencies is provided below:
- [`chrono`](https://crates.io/crates/chrono) v0.4.31 - date and time library, used for representing the different time variables of aircraft
- [`either`](https://crates.io/crates/either) v1.10.0 - general purpose sum type, used for implementing minor (non-important) functionality in the branch-and-bound algorithm
- [`itertools`](https://crates.io/crates/itertools) v0.12.1 - utilities for iterators, used for simplifying problem instance parsing code
- [`rust_xlsxwriter`](https://crates.io/crates/rust_xlsxwriter) v0.63.0 - utilities for creating and modifying Excel files, used for converting problem instances to Excel data
- [`serde`](https://crates.io/crates/serde) v1.0.195 - serialization and deserialization framework, used for parsing problem instances from textual formats
- [`serde_with`](https://crates.io/crates/serde_with) v3.4.0 - additional helper library for `serde`
- [`svg`](https://crates.io/crates/svg) v0.15.0 - utilities for parsing, creating, and modifying SVG files, used for simplifying sequence visualiser code
- [`thiserror`](https://crates.io/crates/thiserror) v1.0.56 - utilities for defining error types, used for simplifying problem instance parsing code

Note that this list does not include transitive dependencies - i.e. dependencies of dependencies.
Neither does it include *development dependencies* - i.e. dependencies that are only used by benchmarks, example code, and tests, but not by the actual implementation.
An exhaustive list of all dependencies can be obtained by running [`cargo tree`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html), or alternatively by checking the Cargo manifests (`Cargo.toml` files) in each sub-directory.

# Building and Running Code

For running the mathematical program:
1. Open the `cplex` directory in CPLEX
2. Create a [Run Configuration](https://www.ibm.com/docs/en/icos/22.1.1?topic=project-populating-executing-run-configuration) with the `runseq.mod` or `test.mod` files
3. Execute the newly created Run Configuration

Note that the `runseq.mod` file contains the actual model and solves a given problem instance once, while the `test.mod` file is a benchmark runner and will run the model `N` times (where `N` is 100 by default).

For building the branch-and-bound program:
1. Navigate to the `runseq` directory by running `cd runseq` or equivalent
2. Run `cargo build --release` to build the entire project once

Since the branch-and-bound project is essentially a library, it cannot be run by itself - only the benchmarks and examples found in the `benches/` and `examples/` sub-directories respectively can be run.

To run a benchmark, run `cargo bench -- foo`, replacing `foo` with the name of the benchmark to be run.
For example, `cargo bench -- furini` will run the benchmarks for all Milan problem instances, while `cargo bench -- heathrow` will run the benchmarks for all Heathrow problem instances.

Note that running benchmarks may take a considerable amount of time since each instance is sampled at least 100 times.
The results of the benchmark can be viewed in the `target/criterion/` sub-directory that is produced upon a successful run.

To run an example, run `cargo bench --release --example foo`, replacing `foo` with the name of the example to be run.
For example, `cargo bench --release --example branch_bound_furini` will run the example defined in `examples/branch_bound_furini.rs`, while `cargo bench --release --example branch_bound_heathrow` will run the example defined in `examples/branch_bound_heathrow.rs`.
An exhaustive list of all examples can be found in the `examples/` sub-directory.

Note that the `--release` flag is important - it instructs `cargo` (and consequentlty `rustc`, the Rust compiler) to enable optimisations during building.
Rust is an Ahead-Of-Time (AOT) compiled language and relies heavily on compiler optimisations.
Running the examples without optimisations enabled might lead to unnaturally high memory usage and runtimes.

# Generating Problem Instances

Before any code can be run, the problem instances must be downloaded and then converted into a format that can be parsed by the mathematical model and branch-and-bound program.
The newly generated data then needs to be placed in a specific location that the code expects.
The examples defined in `examples/generate_furini.rs` and `examples/generate_heathrow.rs` provide a convenient way to do this, by converting problem instance data from the two different airports into a unified [TOML](https://toml.io/en/) and Excel format.

First, the original problem instance data must be placed in a specific directory.
Assuming that both the `cplex/` and `runseq/` directories are under another directory called `foo/`, the following directories and files must be populated:
```
foo/
    cplex/ ...
    runseq/ ...
    instances/
        furini/
            original/
                sep/
                    info_matrix_FPT01.txt.txt
                    info_matrix_FPT02.txt.txt
                    ...
                    info_matrix_FPT12.txt.txt
                FPT01.txt
                FPT02.txt
                ...
                FPT12.txt
            toml/
            xlsx/
        heathrow/
            original/
                flights.csv
                pushback-durations.csv
                runway-configurations.csv
                runway-separations.csv
                taxi-durations.csv
            toml/
            xlsx/
```

Next, navigate to the `runseq/` directory and run `cargo run --release --example generate_furini` and `cargo run --release --example generate_heathrow`.
This will read the data placed in the `instances/furini/original/` and `instances/heathrow/original/` directories respectively, convert them to `.toml` and `.xlsx` files, and place those files in their respective `toml/` and `xlsx/` sub-directories.

The mathematical program and the branch-and-bound program can be used once the problem instance data has been generated. 

Additionally, certain branch-and-bound examples also expect a `stats/` directory for saving the results (objective values, makespans, etc.) of running the algorithm on these problem instances.
This is also required to be in a sibling directory as shown below:
```txt
foo/
    cplex/
        ...
    runseq/
        ...
    instances/
        ...
    stats/
        furini/
            branch-bound/
        heathrow/
            branch-bound/
```
The sub-directories under the `stats/` directory will be automatically populated upon running the `branch_bound_furini` or `branch_bound_heathrow` examples.

The exact directory and file locations mentioned above can also be worked out by checking the file paths in the relevant model or example being run.