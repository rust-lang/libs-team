use color_eyre::eyre;
use generator::Generator;

mod generator;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // note: newline included
    print!("{}", Generator::default().libs_agenda()?);
    Ok(())
}
