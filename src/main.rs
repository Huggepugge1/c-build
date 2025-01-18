use clap::Parser;

mod build;
mod clean;
mod cli;
mod command;
mod includes;
mod init;
mod run;
mod test;

fn main() {
    let command = cli::Cli::parse();

    match match command.command {
        cli::Commands::Build(build) => build::build(&build),
        cli::Commands::Run(build) => run::run(&build),
        cli::Commands::MemoryRun(build) => run::memory_run(&build),
        cli::Commands::Test(build) => test::run::run(&build),
        cli::Commands::MemoryTest(build) => test::run::memory_run(&build),
        cli::Commands::Init(command) => init::init(&command),
        cli::Commands::Clean => clean::clean(),
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
