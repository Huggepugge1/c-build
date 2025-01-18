use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "c-builder",
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
    #[command(about, long_about = Some("Run the tests"))]
    Test(Build),
    #[command(about, long_about = Some("Run the tests with memory testing"))]
    MemoryTest(Build),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let args = Cli::parse_from(&["c-builder", "build"]);
        assert!(matches!(args.command, Commands::Build(_)));
    }

    #[test]
    fn test_build_release() {
        let args = Cli::parse_from(&["c-builder", "build", "--release"]);
        assert!(matches!(args.command, Commands::Build(_)));
    }

    #[test]
    fn test_run() {
        let args = Cli::parse_from(&["c-builder", "run"]);
        assert!(matches!(args.command, Commands::Run(_)));
    }

    #[test]
    fn test_run_release() {
        let args = Cli::parse_from(&["c-builder", "run", "--release"]);
        assert!(matches!(args.command, Commands::Run(_)));
    }

    #[test]
    fn test_memory_run() {
        let args = Cli::parse_from(&["c-builder", "memory-run"]);
        assert!(matches!(args.command, Commands::MemoryRun(_)));
    }

    #[test]
    fn test_memory_run_release() {
        let args = Cli::parse_from(&["c-builder", "memory-run", "--release"]);
        assert!(matches!(args.command, Commands::MemoryRun(_)));
    }

    #[test]
    fn test_init() {
        let args = Cli::parse_from(&["c-builder", "init"]);
        assert!(matches!(args.command, Commands::Init(_)));

        let path = String::from("test");
        let args = Cli::parse_from(&["c-builder", "init", &path]);
        match args.command {
            Commands::Init(init) => assert_eq!(init.path, path),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_clean() {
        let args = Cli::parse_from(&["c-builder", "clean"]);
        assert!(matches!(args.command, Commands::Clean));
    }
}
