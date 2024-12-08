use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(version, about, long_about = Some("Builds the project"))]
    Build(Build),
    #[command(version, about, long_about = Some("Runs the project"))]
    Run(Build),
    #[command(version, about, long_about = Some("Cleans the project"))]
    Clean,
    #[command(version, about, long_about = Some("Displays the version"))]
    Version,
}

pub type Run = Build;

#[derive(Parser, Debug)]
pub struct Build {
    #[arg(short, long, default_value = "src")]
    pub path: String,

    #[arg(short, long, default_value = "c_target/main")]
    pub output: String,

    #[arg(short, long, default_value_t = true)]
    pub debug: bool,

    #[arg(short = 'O', long, default_value_t = 2)]
    pub optimization: u8,

    #[arg(short, long, default_value_t = true)]
    pub warnings: bool,

    #[arg(short = 'P', long, default_value_t = true)]
    pub pedantic: bool,

    #[arg(short, long, default_value = "c2x")]
    pub std: String,
}
