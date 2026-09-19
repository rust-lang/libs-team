use color_eyre::eyre;
use generator::Generator;

mod generator;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(long, short, default_value = "Libs")]
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
    PgEh,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::from_args();
    let generator = Generator::default();
    let agenda = match args.agenda {
        AgendaKind::Libs => generator.libs_agenda()?,
        AgendaKind::PgEh => generator.error_handling_pg_agenda()?,
    };

    // note: newline included
    print!("{}", agenda);
    Ok(())
}
