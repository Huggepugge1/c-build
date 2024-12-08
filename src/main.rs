use clap::Parser;

mod build;
mod clean;
mod cli;
mod command;
mod file;
mod run;

fn main() {
    let command = cli::Cli::parse();

    match match command.command {
        cli::Commands::Build(command) => build::build(&command),
        cli::Commands::Run(command) => run::run(&command),
        cli::Commands::MemoryRun(command) => run::memory_run(&command),
        cli::Commands::Clean => clean::clean(),
        cli::Commands::Version => Ok(Some(format!("c-build {}", env!("CARGO_PKG_VERSION")))),
    } {
        Ok(v) => {
            if let Some(v) = v {
                println!("{}", v);
                std::process::exit(0);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
