use clap::Parser;
use sdd_coverage::config::{Cli, Command};

// @req FR-CLI-001
// @req FR-CLI-002
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
        Command::Serve(args) => {
            println!(
                "requirements={} source={} tests={} port={}",
                args.config.requirements.display(),
                args.config.source.display(),
                args.config.tests.display(),
                args.port,
            );
        }
    }
}
