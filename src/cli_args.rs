use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    #[arg(default_value = "data")]
    pub path: String,

    #[arg(default_value = "3333")]
    pub port: u16,
}
