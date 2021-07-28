use anyhow::Result;
use cli::AgendaKind;
use generator::Generator;

mod cli;
mod generator;

fn main() -> Result<()> {
    let args = cli::Args::from_args();
    let generator = Generator::default();
    let agenda = match args.agenda {
        AgendaKind::Libs => generator.libs_agenda()?,
        AgendaKind::LibsAPI => generator.libs_api_agenda()?,
    };
    println!("{}", agenda);
    Ok(())
}
