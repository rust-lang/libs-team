# File contributors

This tool will list contributors to individual files in the `rust-lang/rust` repository along with the number of contributions made.

## Usage

From this directory, run something like:

```shell
cargo run --release -- --repo-root $path_to_rust
```

where `$path_to_rust` is the path to a local clone of `rust-lang/rust`.

Results can be filtered to a specific subpath:

```shell
cargo run --release -- --repo-root $path_to_rust --subpath library/alloc/src/collections
```

You can also install it as a Cargo tool and use it that way:

```shell
cargo install --path .
cargo list-contributors --repo-root $path_to_rust
```
