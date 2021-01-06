#![feature(once_cell, const_in_array_repeat_expressions)]

use std::{
    collections::BTreeMap,
    env,
    ffi::OsStr,
    fmt,
    path::{Path, PathBuf},
    process::Command,
    io,
    lazy::SyncLazy,
};

use regex::RegexSet;
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

pub fn top_contributors(
    repo_root: impl AsRef<Path>,
    subpath: Option<impl AsRef<Path>>,
    since: impl fmt::Display,
) -> BTreeMap<String, BTreeMap<String, usize>> {
    let repo_root = repo_root.as_ref();

    // First, get the list of individual contributors to each file
    // Then, combine the subpaths to get a combined list of contributors for each
    // This is done _really_ inefficiently so it takes a little while to complete!
    file_contributors(repo_root, subpath, since)
        .into_iter()
        .fold(BTreeMap::new(), |mut map, (path, contributors)| {
            for ancestor in Path::new(&path).ancestors() {
                let map = map
                    .entry(ancestor.to_string_lossy().into_owned())
                    .or_insert_with(|| BTreeMap::new());
    
                for (author, size) in &contributors {
                    let contributions = map
                        .entry(author.clone())
                        .or_insert_with(|| 0);
    
                    *contributions += size;
                }
            }

            map
        })
}

/// Recurse through the `/library` directory, listing contributors
pub fn file_contributors(
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
            let author = entry
                .path()
                .strip_prefix(repo_root)
                .expect("failed to strip path base")
                .to_string_lossy()
                .into_owned();
            
            let log = log(repo_root, entry.path(), &since);

            (author, log)
        })
        .filter(|(_, contributors)| contributors.len() > 0)
        .fold(BTreeMap::new(), |mut map, (path, contributors)| {
            map.insert(path, contributors);
            map
        })
}

// This is just a grab-bag of filters for some changes that might be sweeping refactorings.
static EXCLUDES: SyncLazy<RegexSet> = SyncLazy::new(|| RegexSet::new(&[
    "(?i)rustfmt",
    "(?i)tidy",
    "(?i)doc",
    "(?i)merge",
    "(?i)split",
    "(?i)move",
    "(?i)refactor",
    "(?i)mv std libs to library/",
    "(?i)deny unsafe ops in unsafe fns",
    "(?i)unsafe_op_in_unsafe_fn",
]).expect("failed to compile regex set"));

/// Run `git log` on a given file and return a map of each author with the number of commits made.
fn log(
    repo_root: impl AsRef<Path>,
    dir: impl AsRef<Path>,
    since: impl fmt::Display,
) -> BTreeMap<String, usize> {
    let stdout = Command::new("git")
        .args(&[
            "log",
            "--format=%an:gitsplit:%s",
            "--numstat",
            "--no-merges",
            &format!("--since={}", since),
            "--follow", // follow renames
            dir.as_ref().to_str().expect("non UTF8 path"),
        ])
        .current_dir(repo_root)
        .output()
        .expect("failed to run git blame")
        .stdout;

    let stdout = String::from_utf8(stdout).expect("non UTF8 git blame output");

    stdout
        .lines()
        .filter(|line| !line.is_empty())
        .chunk_by::<2>()
        .filter_map(|lines| {
            let summary = lines[0].expect("missing summary");
            let diff = lines[1].expect("missing diff");

            let mut summary_parts = summary.split(":gitsplit:");
            let author = summary_parts.next().expect("missing author");
            let summary = summary_parts.next().expect("missing summary");
            assert!(summary_parts.next().is_none(), "invalid log line");

            let mut diff_parts = diff.split_whitespace();
            let additions: usize = diff_parts.next().expect("missing additions").parse().expect("failed to parse additions");

            if author != "bors" && !EXCLUDES.is_match(summary) {
                Some((author.to_owned(), additions))
            } else {
                None
            }
        })
        .fold(BTreeMap::new(), |mut map, (author, contribution)| {
            *map.entry(author).or_insert_with(|| 0) += contribution;
            map
        })
}

struct ChunkBy<I, const N: usize>(I);

impl<I, const N: usize> Iterator for ChunkBy<I, N>
where
    I: Iterator,
{
    type Item = [Option<I::Item>; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = [None; N];

        for i in 0..N {
            chunk[i] = self.0.next();
        }

        chunk.iter().any(Option::is_some).then(|| chunk)
    }
}

trait ChunkByExt {
    fn chunk_by<const N: usize>(self) -> ChunkBy<Self, N>
    where
        Self: Sized,
    {
        ChunkBy(self)
    }
}

impl<I> ChunkByExt for I where I: Iterator {}
