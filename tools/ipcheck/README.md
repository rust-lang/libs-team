# IP Check

Originally created for https://github.com/rust-lang/rust/pull/76098, the tool now tests
the *current* implementation of various helper methods against other programming languages.

This is a little utility program for checking the behavior of various language's IP address implementations.
The goal is to make sure the Rust programs are either the same or deliberately different to other languages.

This same approach might be useful for other APIs that have externally specified behavior that may diverge betweens implementations.

## Implementations

These live under the `impls` directory.

- Rust (Current) (`impls/rust_current`) with the current behavior on `nightly`
- .NET (`impls/dotnet`)
- Python (`impls/python`)
- Go (`impls/go`)
- Java (`impls/java`)

There was a Rust implementation with behavior proposed in `#76098` ("Rust (New)")
but the PR didn't go through so the implementation has been removed.

## Running

With the comparison languages available, you can run `cd host && cargo run` to compare them.
The results are written as a Markdown table to `stdout`.
The set of interesting inputs to compare comes from the `/host/input.txt` file.

## How it works

The _host_ program (under `/host`) will attempt to build and execute a number of language implementations.
Each language implementation will accept an input via `stdin` and write a JSON payload to `stdout` containing the results of its execution.
These payloads are then compared against a reference implementation to see how they're affected by different inputs.
