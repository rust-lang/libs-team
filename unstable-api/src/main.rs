use std::{env, path::PathBuf};

use anyhow::Error;
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
    repo_root: PathBuf,
    #[structopt(long)]
    feature: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_iter(env::args().filter(|arg| arg != "unstable-api"));

    let libs = vec![
        {
            let mut lib_core = opt.repo_root.clone();
            lib_core.push("library");
            lib_core.push("core");
            lib_core
        },
        {
            let mut lib_alloc = opt.repo_root.clone();
            lib_alloc.push("library");
            lib_alloc.push("alloc");
            lib_alloc
        },
        {
            let mut lib_std = opt.repo_root.clone();
            lib_std.push("library");
            lib_std.push("std");
            lib_std
        },
    ];

    for crate_root in libs {
        visit::pub_unstable(crate_root, &opt.feature)?;
    }

    Ok(())
}
