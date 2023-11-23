use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(long, short, default_value = "LibsAPI")]
    pub agenda: AgendaKind,
}

impl Args {
    pub fn from_args() -> Args {
        <Args as Parser>::parse()
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "verbatim")]
pub enum AgendaKind {
    Libs,
    LibsAPI,
    PGEH,
}
