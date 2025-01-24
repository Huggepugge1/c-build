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
        cli::Commands::Run(run) => run::run(&run),
        cli::Commands::MemoryRun(run) => run::memory_run(&run),
        cli::Commands::Test(test) => test::run::run(&test),
        cli::Commands::MemoryTest(test) => test::run::memory_run(&test),
        cli::Commands::Init(init) => init::init(&init),
        cli::Commands::Clean => clean::clean(),
    } {
        Ok(v) => {
            if !v.is_empty() {
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
