use std::{env, path::PathBuf};

use anyhow::{bail, Context, Error};
use structopt::StructOpt;

mod util;
mod visit;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "unstable-api",
    about = "Dump the public API for an unstable feature"
)]
struct Opt {
    /// Repository root of `rust-lang/rust`.
    #[structopt(long, parse(from_os_str))]
    repo_root: Option<PathBuf>,
    #[structopt(long)]
    feature: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_iter(env::args().filter(|arg| arg != "unstable-api"));

    let repo_root = match opt.repo_root {
        Some(p) => p,
        None => find_repo_root()?,
    };

    let feature = opt.feature;

    let libs = vec![
        repo_root.clone().join("library/core"),
        repo_root.clone().join("library/alloc"),
        repo_root.clone().join("library/std"),
    ];

    with_output_formatting_maybe(move || {
        for crate_root in libs {
            visit::pub_unstable(crate_root, &feature)?;
        }

        Ok(())
    })
}

fn find_repo_root() -> Result<PathBuf, Error> {
    let path = std::process::Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .context("unable to find repository root")?
        .stdout;
    let mut path = PathBuf::from(String::from_utf8(path)?);
    path.pop();
    Ok(path)
}

#[cfg(not(unix))]
fn with_output_formatting_maybe<F>(f: F) -> Result<(), Error>
where
    F: FnOnce() -> Result<(), Error>,
{
    f()
}

#[cfg(unix)]
fn with_output_formatting_maybe<F>(f: F) -> Result<(), Error>
where
    F: FnOnce() -> Result<(), Error>,
{
    use nix::unistd::{isatty, close, dup, dup2};
    use std::os::unix::io::AsRawFd;
    use std::process::{Command, Stdio};
    use std::io::{stdout, Write};

    let mut original_stdout = None;
    let mut inner_child = None;

    stdout().flush().unwrap(); // just in case

    if isatty(1) == Ok(true) {
        // Pipe the output through `bat` for nice formatting and paging, if available.
        if let Ok(mut bat) = Command::new("bat")
            .arg("--language=rust")
            .arg("--plain") // Disable line numbers for easy copy-pasting.
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
        {
            // Hold on to our stdout for later.
            original_stdout = Some(dup(1).unwrap());
            // Replace our stdout by the pipe into `bat`.
            dup2(bat.stdin.take().unwrap().as_raw_fd(), 1).unwrap();

            inner_child = Some(bat);
        }
    }

    // Pipe the output through `rustfmt`, if available.
    if let Ok(mut rustfmt) = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit()) // This pipes into `bat` if it was executed above.
        .spawn()
    {
        // Hold on to our stdout for later, if we didn't already.
        original_stdout.get_or_insert_with(|| dup(1).unwrap());
        // Replace our stdout by the pipe into `rustfmt`.
        dup2(rustfmt.stdin.take().unwrap().as_raw_fd(), 1).unwrap();

        inner_child.get_or_insert(rustfmt);
    }

    let result = f();

    if let Some(fd) = original_stdout {
        // Overwriting the current stdout with the original stdout
        // closes the pipe to the child's stdin, allowing the child to
        // exit.
        stdout().flush().unwrap(); // just in case
        dup2(fd, 1).unwrap();
        close(fd).unwrap();
    }

    if let Some(mut child) = inner_child {
        // Wait for inner child to exit to ensure it won't write to
        // original stdout after we return.
        if !child.wait()?.success() {
            bail!("output formatting failed");
        }
    }

    result
}
