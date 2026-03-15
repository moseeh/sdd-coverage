use clap::Parser;
use sdd_coverage::config::{Cli, Command};

// @req FR-CLI-001
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan(args) => {
            println!(
                "requirements={} source={} tests={} strict={}",
                args.config.requirements.display(),
                args.config.source.display(),
                args.config.tests.display(),
                args.strict,
            );
        }
    }
}
