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
