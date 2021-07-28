use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long, short, default_value = "LibsAPI", possible_values = &AgendaKind::variants())]
    pub agenda: AgendaKind,
}

impl Args {
    pub fn from_args() -> Args {
        StructOpt::from_args()
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum AgendaKind {
        Libs,
        LibsAPI,
    }
}
