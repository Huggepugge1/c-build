use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "c-build",
    about = "A simple build tool for C projects",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about, long_about = Some("Builds the project"))]
    Build(Build),
    #[command(about, long_about = Some("Runs the project"))]
    Run(Build),
    #[command(about, long_about = Some("Runs the project with memory testing"))]
    MemoryRun(Build),
    #[command(about, long_about = Some("Initializes a new project"))]
    Init(Init),
    #[command(about, long_about = Some("Cleans the project"))]
    Clean,
}

#[derive(Parser, Debug)]
pub struct Build {
    #[arg(short, long, default_value_t = false)]
    pub release: bool,
}

#[derive(Parser, Debug)]
pub struct Init {
    #[arg(default_value = ".")]
    pub path: String,
}
