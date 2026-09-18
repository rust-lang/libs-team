use std::{
    env,
    io::{self, IsTerminal, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Context, Error, anyhow, bail};
use clap::Parser;

mod util;
mod visit;

#[derive(Debug, Parser)]
#[command(
    name = "unstable-api",
    about = "Dump the public API for an unstable feature"
)]
struct Opt {
    /// Repository root of `rust-lang/rust`.
    #[arg(long)]
    repo_root: Option<PathBuf>,
    #[arg(long)]
    feature: String,
}

fn main() -> Result<(), Error> {
    let opt = Opt::parse_from(env::args().filter(|arg| arg != "unstable-api"));

    let repo_root = match opt.repo_root {
        Some(p) => p,
        None => find_repo_root()?,
    };

    let feature = opt.feature;

    let libs = vec![
        repo_root.clone().join("library/core"),
        repo_root.clone().join("library/alloc"),
        repo_root.join("library/std"),
    ];

    let mut output = String::new();
    for crate_root in libs {
        output.push_str(&visit::pub_unstable(crate_root, &feature)?);
    }

    write_output(&output)
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

fn write_output(output: &str) -> Result<(), Error> {
    let output = format_with_rustfmt(output)?.unwrap_or_else(|| output.to_owned());

    if io::stdout().is_terminal()
        && let Ok(mut bat) = Command::new("bat")
            .arg("--language=rust")
            .arg("--plain")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
    {
        bat.stdin
            .take()
            .ok_or_else(|| anyhow!("bat stdin was not available"))?
            .write_all(output.as_bytes())?;
        if !bat.wait()?.success() {
            bail!("output formatting failed");
        }
        return Ok(());
    }

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}

fn format_with_rustfmt(output: &str) -> Result<Option<String>, Error> {
    let mut rustfmt = match Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };

    rustfmt
        .stdin
        .take()
        .ok_or_else(|| anyhow!("rustfmt stdin was not available"))?
        .write_all(output.as_bytes())?;
    let output = rustfmt.wait_with_output()?;
    if !output.status.success() {
        bail!("output formatting failed");
    }

    Ok(Some(String::from_utf8(output.stdout)?))
}
