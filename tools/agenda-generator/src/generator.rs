use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use chrono::{Duration, NaiveDateTime};
use color_eyre::eyre::{Result, WrapErr};
use color_eyre::{Section, SectionExt};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use serde::de::{DeserializeOwned, Deserializer};

#[derive(Default)]
pub struct Generator {
    agenda: String,
    seen: BTreeSet<String>,
}

#[derive(Deserialize)]
struct FcpWithInfo {
    reviews: Vec<(GitHubUser, bool)>,
    concerns: Vec<(String, IssueComment, GitHubUser)>,
    issue: FcpIssue,
    status_comment: IssueComment,
    #[serde(skip)]
    checkboxes: Checkboxes,
}

#[derive(Deserialize, Default)]
struct Checkboxes {
    libs_unchecked: usize,
    libs_and_former_fcp_unchecked: usize,
    other_unchecked: BTreeMap<String, BTreeSet<String>>,
    total_other_unchecked: usize,
}

#[derive(Deserialize)]
struct GitHubUser {
    login: String,
}

#[derive(Deserialize)]
struct FcpIssue {
    number: i32,
    title: String,
    labels: Vec<String>,
    repository: String,
}

enum Shorten {
    Text,
    Href,
}

#[derive(Deserialize)]
struct IssueComment {
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    id: u64,
}

#[derive(Deserialize)]
struct FcpTeam {
    members: Vec<String>,
}

#[derive(Deserialize)]
struct FcpTeams {
    teams: BTreeMap<String, FcpTeam>,
    #[serde(skip)]
    people: BTreeMap<String, BTreeSet<String>>,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct TeamMember {
    github: String,
}
impl Borrow<String> for TeamMember {
    fn borrow(&self) -> &String {
        &self.github
    }
}

#[derive(Deserialize)]
struct Libs {
    members: BTreeSet<TeamMember>,
    alumni: BTreeSet<TeamMember>,
}

#[derive(Deserialize)]
struct Teams {
    libs: Libs,
}

macro_rules! format_with_anchor {
    ($url:expr, $orig_url:expr, $anchor:expr, $kind:expr) => {
        if $anchor.is_empty() {
            $url.to_string()
        } else {
            match $kind {
                Shorten::Text => format!("[{}](https://{}#{})", $url, $url, $anchor),
                Shorten::Href => format!("https://{}#{}", $url, $anchor),
            }
        }
    };
}

fn shorten(url: &str, kind: Shorten) -> String {
    let (url, anchor) = url.split_once("#").unwrap_or((url, ""));
    if let Some(num) = url.strip_prefix("https://github.com/rust-lang/rust/issues/") {
        format_with_anchor!(format_args!("rust.tf/{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/rust/pull/") {
        format_with_anchor!(format_args!("rust.tf/{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/rfcs/issues/") {
        format_with_anchor!(format_args!("rust.tf/rfc{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/rfcs/pull/") {
        format_with_anchor!(format_args!("rust.tf/rfc{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/libs-team/issues/") {
        format_with_anchor!(format_args!("rust.tf/libs{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/libs-team/pull/") {
        format_with_anchor!(format_args!("rust.tf/libs{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/stdarch/issues/") {
        format_with_anchor!(format_args!("rust.tf/stdarch{num}"), orig_url, anchor, kind)
    } else if let Some(num) = url.strip_prefix("https://github.com/rust-lang/stdarch/pull/") {
        format_with_anchor!(format_args!("rust.tf/stdarch{num}"), orig_url, anchor, kind)
    } else if let Some(url) = url.strip_prefix("https://") {
        format_with_anchor!(url, orig_url, anchor, kind)
    } else {
        format_with_anchor!(url, orig_url, anchor, kind)
    }
}

impl Generator {
    pub fn libs_agenda(mut self) -> Result<String> {
        writeln!(
            &mut self.agenda,
            "# Libs Meeting {}

###### tags: `Libs Meetings` `Minutes`

**Meeting Link**: https://meet.jit.si/rust-libs-meeting-crxoz2at8hiccp7b3ixf89qgxfymlbwr
**Attendees**: ...

## Agenda

- Nominated items
- Regressions
- FCPs (optional)
- ACPs (optional)
- Anything else?

",
            chrono::Utc::now().format("%Y-%m-%d")
        )?;

        writeln!(
            &mut self.agenda,
            "## Triage

",
        )?;

        GithubQuery::new("Critical")
            .labels(&["T-libs", "P-critical"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Backports")
            .labels(&["T-libs", "stable-nominated"])
            .labels(&["T-libs", "beta-nominated"])
            .exclude_labels(&["beta-accepted"])
            .state(State::Any)
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Prioritization Requested")
            .labels(&["T-libs", "I-prioritize"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("Nominated")
            .labels(&["I-libs-nominated"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .repo("rust-lang/libs-team")
            .repo("rust-lang/stdarch")
            .write(&mut self)?;

        GithubQuery::new("Waiting on Team")
            .labels(&["S-waiting-on-t-libs"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        GithubQuery::new("needs decision")
            .labels(&["T-libs", "I-needs-decision"])
            .repo("rust-lang/rust")
            .write(&mut self)?;

        GithubQuery::new("Regressions")
            .labels(&["T-libs", "regression-untriaged"])
            .labels(&["T-libs", "regression-from-stable-to-stable"])
            .labels(&["T-libs", "regression-from-stable-to-beta"])
            .labels(&["T-libs", "regression-from-stable-to-nightly"])
            .exclude_labels(&["I-libs-nominated"])
            .exclude_labels(&["P-low"])
            .repo("rust-lang/rust")
            .repo("rust-lang/rfcs")
            .write(&mut self)?;

        self.fcps(String::from("T-libs"))?;

        writeln!(
            &mut self.agenda,
            "## ACPs

",
        )?;

        let new_proposals = GithubQuery::new("New")
            .labels(&["api-change-proposal"])
            .exclude_labels(&["ACP-accepted"])
            .repo("rust-lang/libs-team")
            .sort(Sort::Newest)
            .take(10)
            .rev(true)
            .write(&mut self)?;

        GithubQuery::new("Stalled")
            .labels(&["api-change-proposal"])
            .exclude_labels(&["ACP-accepted"])
            .repo("rust-lang/libs-team")
            .sort(Sort::LeastRecentlyUpdated)
            .take(10)
            .skip(new_proposals.iter())
            .shuffle(true)
            .write(&mut self)?;

        writeln!(
            &mut self.agenda,
            "_Generated by [fully-automatic-rust-libs-team-triage-meeting-agenda-generator](https://github.com/rust-lang/libs-team/tree/main/tools/agenda-generator)_",
        )?;
        Ok(self.agenda)
    }

    pub fn error_handling_pg_agenda(mut self) -> Result<String> {
        writeln!(
            &mut self.agenda,
            "# Project Group Error Handling Meeting {}

###### tags: `Error Handling` `Minutes`

**Attendees**: ...

## Agenda Items

- [Open action items](https://hackmd.io/@rust-libs/Hyj7kRSld)
- Triage
- Individual Status Updates

## Triage
",
            chrono::Utc::now().format("%Y-%m-%d")
        )?;

        GithubQuery::new("Nominated")
            .labels(&["PG-error-handling", "I-nominated"])
            .repo("rust-lang/rust")
            .repo("rust-lang/project-error-handling")
            .write(&mut self)?;

        GithubQuery::new("PG Error Handling")
            .labels(&["PG-error-handling"])
            .repo("rust-lang/rust")
            .repo("rust-lang/project-error-handling")
            .write(&mut self)?;

        GithubQuery::new("Area Error Handling")
            .labels(&["A-error-handling"])
            .repo("rust-lang/rust")
            .write(&mut self)?;

        GithubQuery::new("PG Error Handling")
            .repo("rust-lang/project-error-handling")
            .write(&mut self)?;

        writeln!(&mut self.agenda,
        "## Actions

- [ ] Reply to all issues/PRs discussed in this meeting, or add them to the [open action items](https://hackmd.io/UrERZvi5RwyxfGvo-RtC6g).
",
    )?;

        writeln!(
            &mut self.agenda,
            "_Generated by [fully-automatic-rust-libs-team-triage-meeting-agenda-generator](https://github.com/rust-lang/libs-team/tree/main/tools/agenda-generator)_"
        )?;
        Ok(self.agenda)
    }

    fn fcps(&mut self, label: String) -> Result<()> {
        let fcps: Vec<FcpWithInfo> = reqwest::blocking::get("https://rfcbot.rs/api/all")?.json()?;

        let mut fcp_teams: FcpTeams =
            reqwest::blocking::get("https://team-api.infra.rust-lang.org/v1/rfcbot.json")?
                .json()?;
        for (team_name, team) in &fcp_teams.teams {
            for person in &team.members {
                fcp_teams
                    .people
                    .entry(person.clone())
                    .or_default()
                    .insert(team_name.to_owned());
            }
        }

        let teams: Teams =
            reqwest::blocking::get("https://team-api.infra.rust-lang.org/v1/teams.json")?.json()?;

        self.write_fcps(label, fcps, fcp_teams, teams)
    }

    fn write_fcps(
        &mut self,
        label: String,
        mut fcps: Vec<FcpWithInfo>,
        fcp_teams: FcpTeams,
        teams: Teams,
    ) -> Result<()> {
        fcps.retain(|fcp| fcp.issue.labels.contains(&label));

        // Don't filter out FCPs.
        if false {
            let waiting_on_author = "S-waiting-on-author".to_string();
            fcps.retain(|fcp| !fcp.issue.labels.contains(&waiting_on_author));
            let now = chrono::Utc::now().naive_utc();
            fcps.retain(|fcp| {
                let created = fcp.status_comment.created_at;
                let updated = fcp.status_comment.updated_at;
                (now - created) > Duration::weeks(4) && (now - updated) > Duration::days(5)
            });
        }

        // get count of checkboxes needed for libs and other teams
        for fcp in &mut fcps {
            for (reviewer, review) in &fcp.reviews {
                let reviewer = &reviewer.login;
                if *review {
                    continue;
                }
                let reviewer_teams = fcp_teams.people.get(reviewer);

                let team_name = reviewer_teams
                    .and_then(|teams| {
                        if teams.contains("T-libs") {
                            Some(String::from("A-libs"))
                        } else {
                            // require teams to match labels for PR, otherwise
                            // it's noisy since people are on multiple teams
                            teams
                                .iter()
                                .find(|team| fcp.issue.labels.contains(team))
                                .cloned()
                        }
                    })
                    .or_else(|| {
                        // if someone isn't on any FCP team, but is on the libs team or an alum,
                        // assume they're an old libs-fcp member
                        (teams.libs.members.contains(reviewer)
                            || teams.libs.alumni.contains(reviewer))
                        .then(|| String::from("A-libs-former-fcp"))
                    })
                    .unwrap_or_else(|| String::from("Z-other"));
                if let Some(name) = team_name.strip_prefix("A-") {
                    if name == "libs" {
                        fcp.checkboxes.libs_unchecked += 1;
                    }
                    fcp.checkboxes.libs_and_former_fcp_unchecked += 1;
                } else {
                    fcp.checkboxes.total_other_unchecked += 1;
                }
                fcp.checkboxes
                    .other_unchecked
                    .entry(team_name)
                    .or_default()
                    .insert(reviewer.clone());
            }
        }

        fcps.sort_by_key(|fcp| {
            (
                // move all cases where libs has nothing left to check, and there are no concerns, to the end
                fcp.checkboxes.libs_and_former_fcp_unchecked == 0 && fcp.concerns.is_empty(),
                // then, sort by number of concerns ascending (starting at 0)
                fcp.concerns.len(),
                // prefer things that libs can check boxes for
                Reverse(fcp.checkboxes.libs_and_former_fcp_unchecked > 0),
                // prefer things where libs checking boxes would achieve N-2 threshold
                Reverse(fcp.checkboxes.total_other_unchecked <= 2),
                // sort by the number of remaining checkboxes, ascending
                fcp.checkboxes.libs_and_former_fcp_unchecked + fcp.checkboxes.total_other_unchecked,
                // prioritize ones where libs has the most checkboxes of the total
                Reverse(fcp.checkboxes.libs_unchecked),
                // prioritize ones where also the former FCP team has checkboxes
                Reverse(fcp.checkboxes.libs_and_former_fcp_unchecked),
            )
        });

        writeln!(self.agenda, "## FCPs")?;
        writeln!(self.agenda)?;

        for fcp in &fcps {
            let url = shorten(
                &format!(
                    "https://github.com/{}/issues/{}#issuecomment-{}",
                    fcp.issue.repository, fcp.issue.number, fcp.status_comment.id,
                ),
                Shorten::Text,
            );
            writeln!(self.agenda, "#### {url} {}", escape(&fcp.issue.title))?;

            if fcp.checkboxes.libs_and_former_fcp_unchecked == 0
                && fcp.checkboxes.total_other_unchecked == 0
            {
                writeln!(self.agenda, "- has all checkboxes")?;
            } else {
                for (team_name, team) in &fcp.checkboxes.other_unchecked {
                    let team_name = team_name
                        .split_once("-")
                        .map_or(&**team_name, |(_, after)| after);
                    let count = team.len();
                    write!(self.agenda, "- needs {count} {team_name}:")?;
                    for reviewer in team {
                        write!(
                            self.agenda,
                            " [@{reviewer}](https://rfcbot.rs/fcp/{reviewer})"
                        )?;
                    }
                    writeln!(self.agenda)?;
                }
            }

            for (concern, comment, _) in &fcp.concerns {
                writeln!(
                    self.agenda,
                    "- blocked by concern: [{concern}]({})",
                    shorten(
                        &format!(
                            "https://github.com/{}/issues/{}#issuecomment-{}",
                            fcp.issue.repository, fcp.issue.number, comment.id
                        ),
                        Shorten::Href
                    ),
                )?;
            }

            writeln!(self.agenda)?;
        }

        writeln!(self.agenda)?;

        Ok(())
    }

    fn write_issues(&mut self, issues: &[Issue]) -> Result<()> {
        for issue in issues.iter().rev() {
            write!(
                self.agenda,
                "#### {}",
                shorten(&issue.html_url, Shorten::Text)
            )?;
            for label in issue.labels.iter().filter(|s| s.starts_with("P-")) {
                write!(self.agenda, " `{}`", label)?;
            }
            writeln!(self.agenda, " {}", escape(&issue.title).trim())?;
            if issue
                .labels
                .iter()
                .any(|l| l == "finished-final-comment-period")
            {
                write!(self.agenda, "FCP finished.")?;
                for label in issue.labels.iter() {
                    if let Some(disposition) = label.strip_prefix("disposition-") {
                        write!(self.agenda, " Should be {}d?", disposition)?;
                    }
                }
                writeln!(self.agenda)?;
            }
            writeln!(self.agenda)?;
        }

        Ok(())
    }

    fn dedup(&mut self, issues: Vec<Issue>) -> impl Iterator<Item = Issue> + '_ {
        issues
            .into_iter()
            .filter(move |issue| self.seen.insert(issue.html_url.clone()))
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Sort {
    Newest,
    Oldest,
    MostCommented,
    LeastCommented,
    MostRecentlyUpdated,
    LeastRecentlyUpdated,
}

impl Sort {
    fn api_str(&self) -> &'static str {
        match self {
            Sort::Newest => "&sort=created&direction=desc",
            Sort::Oldest => "&sort=created&direction=asc",
            Sort::MostCommented => "&sort=comments&direction=asc",
            Sort::LeastCommented => "&sort=comments&direction=desc",
            Sort::MostRecentlyUpdated => "&sort=updated&direction=desc",
            Sort::LeastRecentlyUpdated => "&sort=updated&direction=asc",
        }
    }

    fn web_ui_str(&self) -> &'static str {
        match self {
            Sort::Newest => "+sort:created-desc",
            Sort::Oldest => "+sort:created-asc",
            Sort::MostCommented => "+sort:comments-desc",
            Sort::LeastCommented => "+sort:comments-asc",
            Sort::MostRecentlyUpdated => "+sort:updated-desc",
            Sort::LeastRecentlyUpdated => "+sort:updated-asc",
        }
    }
}

struct GithubQuery {
    name: &'static str,
    labels: Vec<&'static [&'static str]>,
    excluded_labels: Vec<&'static [&'static str]>,
    repos: Vec<&'static str>,
    sort: Option<Sort>,
    count: Option<usize>,
    to_skip: Vec<String>,
    shuffle: bool,
    rev: bool,
    state: State,
}

#[allow(dead_code)]
enum State {
    Open,
    Closed,
    Any,
}

impl State {
    fn api_str(&self) -> &'static str {
        match self {
            State::Open => "&state=open",
            State::Closed => "&state=closed",
            State::Any => "&state=all",
        }
    }

    fn web_ui_str(&self) -> &'static str {
        match self {
            State::Open => "+is:open",
            State::Closed => "+is:closed",
            State::Any => "",
        }
    }
}

impl GithubQuery {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            labels: vec![],
            excluded_labels: vec![],
            repos: vec![],
            sort: None,
            to_skip: vec![],
            count: None,
            shuffle: false,
            rev: false,
            state: State::Open,
        }
    }

    fn labels(&mut self, labels: &'static [&'static str]) -> &mut Self {
        self.labels.push(labels);
        self
    }

    fn exclude_labels(&mut self, labels: &'static [&'static str]) -> &mut Self {
        self.excluded_labels.push(labels);
        self
    }

    fn repo(&mut self, repo: &'static str) -> &mut Self {
        self.repos.push(repo);
        self
    }

    fn sort(&mut self, sort: Sort) -> &mut Self {
        self.sort = Some(sort);
        self
    }

    fn shuffle(&mut self, shuffle: bool) -> &mut Self {
        self.shuffle = shuffle;
        self
    }

    fn skip<'a>(&mut self, to_skip: impl Iterator<Item = &'a Issue>) -> &mut Self {
        self.to_skip = to_skip.map(|i| i.html_url.clone()).collect();
        self
    }

    fn take(&mut self, count: usize) -> &mut Self {
        self.count = Some(count);
        self
    }

    fn rev(&mut self, rev: bool) -> &mut Self {
        self.rev = rev;
        self
    }

    fn state(&mut self, state: State) -> &mut Self {
        self.state = state;
        self
    }

    fn write(&mut self, generator: &mut Generator) -> Result<Vec<Issue>> {
        let mut all_issues = Vec::new();

        let mut written_category = false;

        for repo in &self.repos {
            for labels in &self.labels {
                let cs_labels = labels.join(",");
                let mut endpoint = format!("repos/{}/issues?labels={}", repo, cs_labels);

                endpoint += self.state.api_str();

                if let Some(sort) = self.sort {
                    endpoint += sort.api_str();
                }

                let issues = github_api(&endpoint)?;
                let issues = generator.dedup(issues);

                let issues = issues.filter(|issue| {
                    !self.excluded_labels.iter().any(|labels| {
                        labels
                            .iter()
                            .all(|&label| issue.labels.iter().any(|x| x == label))
                    })
                });
                let issues = issues.filter(|i| !self.to_skip.contains(&i.html_url));

                let mut issues: Vec<_> = issues.collect();

                if issues.is_empty() {
                    continue;
                }

                let url_labels = labels
                    .iter()
                    .map(|label| format!("label:{}", label))
                    .join("+");

                let mut url = format!("https://github.com/{}/issues?q={}", repo, url_labels);

                url += self.state.web_ui_str();

                if let Some(sort) = self.sort {
                    url += sort.web_ui_str();
                }

                //writeln!(
                //    generator.agenda,
                //    "- [{} `{repo}` `{labels}` items]({url})",
                //    issues.len(),
                //    repo = repo,
                //    labels = labels.join("` `"),
                //    url = url,
                //)?;

                if self.shuffle {
                    issues.shuffle(&mut thread_rng());
                }

                if let Some(count) = self.count {
                    issues.truncate(count);
                }
                if self.rev {
                    issues.reverse();
                }
                if !written_category {
                    let category = self.name;
                    writeln!(
                        generator.agenda,
                        "### {category}
",
                    )?;
                    written_category = true;
                }
                generator.write_issues(&issues)?;
                all_issues.append(&mut issues);
            }
        }

        if written_category {
            writeln!(generator.agenda)?;
        }

        Ok(all_issues)
    }
}

#[derive(Debug, Deserialize)]
struct Issue {
    #[allow(dead_code)]
    number: u32,
    html_url: String,
    title: String,
    #[serde(deserialize_with = "deserialize_labels")]
    labels: Vec<String>,
}

fn escape(v: &str) -> String {
    let mut s = String::with_capacity(v.len() + 10);
    let mut inside_code = false;
    v.chars().for_each(|c| {
        match c {
            '`' => inside_code = !inside_code,
            '_' | '*' | '\\' | '[' | ']' | '-' | '<' | '>' if !inside_code => s.push('\\'),
            _ => {}
        }
        s.push(c);
    });
    s
}

fn github_api<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
    let url = format!("https://api.github.com/{}", endpoint);
    let mut client = reqwest::blocking::Client::new()
        .get(&url)
        .header(USER_AGENT, "rust-lang libs agenda maker");
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        client = client.header(AUTHORIZATION, format!("token {}", token));
    }
    let response = client.send()?;
    let response = response.text()?;
    serde_json::from_str(&response)
        .wrap_err("response body cannot be deserialized")
        .with_section(|| response.header("Response:"))
}

fn deserialize_labels<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    #[derive(Debug, Deserialize)]
    struct Label {
        name: String,
    }
    let v = Vec::<Label>::deserialize(d)?;
    Ok(v.into_iter().map(|l| l.name).collect())
}
