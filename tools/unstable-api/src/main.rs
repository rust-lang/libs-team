use std::{env, path::PathBuf};

use anyhow::{Context, Error};
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

    #[cfg(unix)]
    setup_output_formatting();

    let libs = vec![
        repo_root.clone().join("library/core"),
        repo_root.clone().join("library/alloc"),
        repo_root.clone().join("library/std"),
    ];

    for crate_root in libs {
        visit::pub_unstable(crate_root, &opt.feature)?;
    }

    Ok(())
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

#[cfg(unix)]
fn setup_output_formatting() {
    use nix::unistd::{isatty, dup2};
    use std::os::unix::io::AsRawFd;
    use std::process::{Command, Stdio};

    if isatty(1) == Ok(true) {
        // Pipe the output through `bat` for nice formatting and paging, if available.
        if let Ok(mut bat) = Command::new("bat")
            .arg("--language=rust")
            .arg("--plain") // Disable line numbers for easy copy-pasting.
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
        {
            // Replace our stdout by the pipe into `bat`.
            dup2(bat.stdin.take().unwrap().as_raw_fd(), 1).unwrap();
        }
    }

    // Pipe the output through `rustfmt`, if available.
    if let Ok(mut rustfmt) = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit()) // This pipes into `bat` if it was executed above.
        .spawn()
    {
        // Replace our stdout by the pipe into `rustfmt`.
        dup2(rustfmt.stdin.take().unwrap().as_raw_fd(), 1).unwrap();
    }
}
