use cli::AgendaKind;
use color_eyre::eyre;
use generator::Generator;

mod cli;
mod generator;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = cli::Args::from_args();
    let generator = Generator::default();
    let agenda = match args.agenda {
        AgendaKind::Libs => generator.libs_agenda()?,
        AgendaKind::LibsACP => generator.libs_acp_agenda()?,
        AgendaKind::LibsAPI => generator.libs_api_agenda()?,
        AgendaKind::PGEH => generator.error_handling_pg_agenda()?,
    };
    println!("{}", agenda);
    Ok(())
}
