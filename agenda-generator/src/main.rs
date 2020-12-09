#![feature(str_split_once)]

use anyhow::{anyhow, Result};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::de::{DeserializeOwned, Deserializer};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

fn main() -> Result<()> {
    println!(
        "# Libs Meeting {}

###### tags: `Libs Meetings` `Minutes`

**Attendees**: ...

## Agenda

- Triage
- Anything else?

## Triage
",
        chrono::Utc::now().format("%Y-%m-%d")
    );

    show_fcps()?;

    show_issues()?;

    println!(
        "## Notes

...

## Actions

- [ ] Finish action items from the last meeting.
- [ ] Reply to all issues/PRs discussed in this meeting.
"
    );
    Ok(())
}

fn show_fcps() -> Result<()> {
    let p = reqwest::blocking::get("https://rfcbot.rs")?.text()?;
    let mut p = p.lines();
    p.find(|s| s.trim_end() == "<h4><code>T-libs</code></h4>")
        .ok_or_else(|| anyhow!("Missing T-libs section"))?;

    let mut fcps = BTreeMap::<&str, Vec<Fcp>>::new();
    let mut reviewer_count: BTreeMap<&str, usize> = [
        "Amanieu",
        "BurntSushi",
        "dtolnay",
        "KodrAus",
        "m-ou-se",
        "sfackler",
        "withoutboats",
    ]
    .iter()
    .map(|&r| (r, 0))
    .collect();

    loop {
        let line = p.next().unwrap();
        if line.starts_with("<h4>") {
            break;
        }
        if line.trim() == "<li>" {
            let disposition = p.next().unwrap().trim().strip_suffix(":").unwrap();
            let url = p
                .next()
                .unwrap()
                .trim()
                .strip_prefix("<b><a href=\"")
                .unwrap()
                .strip_suffix('"')
                .unwrap();
            assert_eq!(p.next().unwrap().trim(), "target=\"_blank\">");
            let title_and_number = p.next().unwrap().trim().strip_suffix(")</a></b>").unwrap();
            let (title, number) = title_and_number.rsplit_once(" (").unwrap();
            let (repo, number) = number.split_once('#').unwrap();
            let mut reviewers = Vec::new();
            let mut concerns = false;
            loop {
                let line = p.next().unwrap().trim();
                if line == "</li>" {
                    break;
                }
                if line == "pending concerns" {
                    concerns = true;
                } else if let Some(line) = line.strip_prefix("<a href=\"/fcp/") {
                    let reviewer = line.split_once('"').unwrap().0;
                    if let Some(n) = reviewer_count.get_mut(reviewer) {
                        reviewers.push(reviewer);
                        *n += 1;
                    }
                }
            }
            fcps.entry(repo).or_default().push(Fcp {
                title,
                repo,
                number,
                disposition,
                url,
                reviewers,
                concerns,
            });
        }
    }

    println!("### FCPs");
    println!();
    println!(
        "{} open T-libs FCPs:",
        fcps.values().map(|v| v.len()).sum::<usize>()
    );
    for (repo, fcps) in fcps.iter() {
        println!("<details><summary><a href=\"https://github.com/{}/issues?q=is%3Aopen+label%3AT-libs+label%3Aproposed-final-comment-period\">{} <code>{}</code> FCPs</a></summary>\n", repo, fcps.len(), repo);
        for fcp in fcps {
            print!(
                "  - [[{} {}]({})] *{}*",
                fcp.disposition,
                fcp.number,
                fcp.url,
                escape(fcp.title)
            );
            println!(" - ({} checkboxes left)", fcp.reviewers.len());
            if fcp.concerns {
                println!("    Blocked on an open concern.");
            }
        }
        println!("</details>");
    }
    println!("<p></p>\n");

    for (i, (&reviewer, &num)) in reviewer_count.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!(
            "[{} ({})](https://rfcbot.rs/fcp/{})",
            reviewer, num, reviewer
        );
    }
    println!();
    println!();

    Ok(())
}

fn show_issues() -> Result<()> {
    let mut seen = BTreeSet::new();

    let mut dedup = |mut issues: Vec<Issue>| -> Vec<Issue> {
        issues.retain(|issue| seen.insert(issue.html_url.clone()));
        issues
    };

    let nominated_issues: Vec<Issue> = dedup(github_api(
        "repos/rust-lang/rust/issues?labels=T-libs,I-nominated",
    )?);
    let nominated_rfcs: Vec<Issue> = dedup(github_api(
        "repos/rust-lang/rfcs/issues?labels=T-libs,I-nominated",
    )?);
    let waiting_on_team_issues: Vec<Issue> = dedup(github_api(
        "repos/rust-lang/rust/issues?labels=T-libs,S-waiting-on-team",
    )?);
    let waiting_on_team_rfcs: Vec<Issue> = dedup(github_api(
        "repos/rust-lang/rfcs/issues?labels=T-libs,S-waiting-on-team",
    )?);
    let needs_decision_issues: Vec<Issue> = dedup(github_api(
        "repos/rust-lang/rust/issues?labels=T-libs,I-needs-decision",
    )?);

    let print_issues = |issues: &[Issue]| {
        for issue in issues.iter().rev() {
            println!(
                "  - [[{}]({})] *{}*",
                issue.number,
                issue.html_url,
                escape(&issue.title)
            );
            if issue.labels.iter().any(|l| l == "finished-final-comment-period") {
                print!("    FCP finished.");
                for label in issue.labels.iter() {
                    if let Some(disposition) = label.strip_prefix("disposition-") {
                        print!(" Should be {}d?", disposition);
                    }
                }
                println!();
            }
        }
    };

    println!("### Nominated");
    println!();
    println!("- [{} `rust-lang/rfcs` items](https://github.com/rust-lang/rfcs/issues?q=is%3Aopen+label%3AT-libs+label%3AI-nominated)", nominated_rfcs.len());
    print_issues(&nominated_rfcs);
    println!("- [{} `rust-lang/rust` items](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AT-libs+label%3AI-nominated)", nominated_issues.len());
    print_issues(&nominated_issues);
    println!();

    println!("### Waiting on team");
    println!();
    println!("- [{} `rust-lang/rfcs` items](https://github.com/rust-lang/rfcs/issues?q=is%3Aopen+label%3AT-libs+label%3AS-waiting-on-team)", waiting_on_team_rfcs.len());
    print_issues(&waiting_on_team_rfcs);
    println!("- [{} `rust-lang/rust` items](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AT-libs+label%3AS-waiting-on-team)", waiting_on_team_issues.len());
    print_issues(&waiting_on_team_issues);
    println!();

    println!("### Needs decision");
    println!();
    println!("- [{} `rust-lang/rust` items](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AT-libs+label%3AI-needs-decision)", needs_decision_issues.len());
    print_issues(&needs_decision_issues);

    println!();

    Ok(())
}

#[derive(Debug)]
struct Fcp<'a> {
    title: &'a str,
    repo: &'a str,
    number: &'a str,
    disposition: &'a str,
    url: &'a str,
    reviewers: Vec<&'a str>,
    concerns: bool,
}

#[derive(Debug, Deserialize)]
struct Issue {
    number: u32,
    html_url: String,
    title: String,
    #[serde(deserialize_with = "deserialize_labels")]
    labels: Vec<String>,
}

fn escape(v: &str) -> String {
    let mut s = String::with_capacity(v.len() + 10);
    v.chars().for_each(|c| {
        match c {
            '_' | '*' | '\\' | '[' | ']' | '-' | '<' | '>' | '`' => s.push('\\'),
            _ => {}
        }
        s.push(c);
    });
    s
}

fn github_api<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
    let mut client = reqwest::blocking::Client::new()
        .get(&format!("https://api.github.com/{}", endpoint))
        .header(USER_AGENT, "rust-lang libs agenda maker");
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        client = client.header(AUTHORIZATION, format!("token {}", token));
    }
    let response = client.send()?;
    Ok(response.json()?)
}

fn deserialize_labels<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    #[derive(Debug, Deserialize)]
    struct Label {
        name: String,
    }
    let v = Vec::<Label>::deserialize(d)?;
    Ok(v.into_iter().map(|l| l.name).collect())
}
