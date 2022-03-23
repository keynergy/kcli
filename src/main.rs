mod config;
mod keyboards;
//mod load_metrics;
mod setup;
use clap::{Parser, Subcommand};
pub use config::Config;
pub use keyboards::{ansi, matrix};
pub use setup::setup;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap[propagate_version = true]]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up data directory
    Setup { dir: Option<String> },
    Corpus {
        #[clap(subcommand)]
        command: CorpusCommand,
    },
}

#[derive(Debug, Subcommand)]
enum CorpusCommand {
    List,
    Use { corpus: String },
    Load { file: PathBuf },
}

fn do_setup() -> Config {
    println!("No data found, setting up.");
    setup::setup(&None)
}

fn main() {
    let cli = Cli::parse();

    let cfg: Config = match confy::load("keynergy") {
        Ok(c) => c,
        Err(_) => do_setup(),
    };
    match &cli.command {
        Commands::Setup { dir } => {
            setup(dir);
        }
        Commands::Corpus { command } => {
            println!("{:?}", command);
        }
    }
}
