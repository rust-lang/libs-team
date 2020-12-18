use std::{
    collections::BTreeMap,
    env,
    ffi::OsStr,
    fmt,
    path::{Path, PathBuf},
    process::Command,
    io,
};

use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "list-contributors",
    about = "List contributors for each library file in `rust-lang/rust`"
)]
struct Opt {
    /// Repository root of `rust-lang/rust`.
    #[structopt(long, parse(from_os_str))]
    repo_root: PathBuf,
    /// Filter to a subpath.
    #[structopt(long)]
    subpath: Option<PathBuf>,
    /// Filter to recent commits.
    #[structopt(long, default_value = "12.months")]
    since: String,
}

fn main() {
    let opt = Opt::from_iter(env::args().filter(|arg| arg != "list-contributors"));

    let top_contributors = top_contributors(opt.repo_root, opt.subpath, opt.since);

    let stdout = io::stdout();
    let stdout = stdout.lock();

    drop(serde_json::to_writer_pretty(stdout, &top_contributors));
}

/// Recurse through the `/library` directory, listing contributors
pub fn top_contributors(
    repo_root: impl AsRef<Path>,
    subpath: Option<impl AsRef<Path>>,
    since: impl fmt::Display,
) -> BTreeMap<String, BTreeMap<String, usize>> {
    let repo_root = repo_root.as_ref();
    let lib_root = {
        let mut lib_root = repo_root.to_owned();
        lib_root.push("library");
        lib_root
    };

    WalkDir::new(lib_root)
        .into_iter()
        .filter_entry(|entry| {
            // Traverse directories unless they're for tests or benches
            if entry.path().is_dir() {
                let is_src = entry
                    .path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|stem| !stem.contains("test") && !stem.contains("bench"))
                    .unwrap_or(false);
                
                is_src
            }
            // Include files if they're Rust source and not tests
            else {
                let is_rs = entry.path().extension() == Some(OsStr::new("rs"));
                let is_src = entry.path().file_stem() != Some(OsStr::new("tests"));
                let is_in_subpath = subpath
                    .as_ref()
                    .map(|subpath| {
                        entry
                            .path()
                            .strip_prefix(repo_root)
                            .expect("failed to strip path base")
                            .starts_with(subpath.as_ref())
                    })
                    .unwrap_or(true);

                is_rs && is_src && is_in_subpath
            }
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| {
            (
                entry
                    .path()
                    .strip_prefix(repo_root)
                    .expect("failed to strip path base")
                    .to_string_lossy()
                    .into_owned(),
                blame(repo_root, entry.path(), &since),
            )
        })
        .filter(|(_, contributors)| contributors.len() > 0)
        .fold(BTreeMap::new(), |mut map, (path, contributors)| {
            map.insert(path, contributors);
            map
        })
}

/// Run `git blame` on a given file and return a map of each author with the number of commits made.
fn blame(
    repo_root: impl AsRef<Path>,
    dir: impl AsRef<Path>,
    since: impl fmt::Display,
) -> BTreeMap<String, usize> {
    let stdout = Command::new("git")
        .args(&[
            "blame",
            "--line-porcelain",
            &format!("--since={}", since),
            dir.as_ref().to_str().expect("non UTF8 path"),
        ])
        .current_dir(repo_root)
        .output()
        .expect("failed to run git blame")
        .stdout;

    let stdout = String::from_utf8(stdout).expect("nont UTF8 git blame output");
    let prefix = "author ";

    stdout
        .lines()
        .filter(|line| line.starts_with(prefix) && !line.contains("bors"))
        .map(|line| line[prefix.len()..].to_owned())
        .fold(BTreeMap::new(), |mut map, author| {
            *map.entry(author).or_insert_with(|| 0) += 1;
            map
        })
}
