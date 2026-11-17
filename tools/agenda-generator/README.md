# Agenda Maker

Creates an agenda for a Libs meeting with current triage items and room for other topics.

## Usage

With Nix and direnv installed, enable the development shell once:

```shell
echo 'use flake path:./nix' > .envrc
direnv allow
```

Alternatively, enter it manually with `nix develop path:./nix`.
The shell provides Rust, OpenSSL, pkg-config, and the GitHub CLI.
Authenticate with `gh auth login` (or set `GITHUB_TOKEN`) before running:

```shell
cargo run -- --agenda Libs
```

