mod config;
mod keyboards;
mod load_metrics;
mod corpus;
mod data;
mod setup;
use clap::{Parser, Subcommand};
pub use config::Config;
use console;
use ctrlc;
use keynergy::Analyzer;
pub use data::Data;
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
    /// Manage stored corpora
    Corpus {
        #[clap(subcommand)]
        command: CorpusCommand,
    },
    /// Load metrics for each keyboard, needed for analysis
    LoadMetrics 
}

#[derive(Debug, Subcommand)]
enum CorpusCommand {
    /// List the stored corpora
    List,
    /// Set the default corpora
    Default,
    /// Load a text file as a new corpus
    Load { file: PathBuf },
    /// Remove a corpus from the list
    Remove,
}

fn main() {
    let mut cfg: Config = confy::load("keynergy").unwrap();

    let mut just_set_up = false;
    if cfg.data_dir == PathBuf::from("") {
        println!("Data dir not found, setting up.");
        setup(&None);
        just_set_up = true;
    }

    let cli = Cli::parse();
    let mut data = Data::load(&cfg);

    let _ = ctrlc::set_handler(move || {
        let term = console::Term::stdout();
        let _ = term.show_cursor();
        std::process::exit(1);
    });

    match &cli.command {
        Commands::Setup { dir } => {
	    if !just_set_up {
                setup(dir);
	    }
        }
        Commands::Corpus { command } => match command {
	    CorpusCommand::List => corpus::list(&data),
	    CorpusCommand::Load { file } => {
                corpus::load(&mut data, file);
                println!("Writing data...");
                data.save(&cfg);
                println!("Done!");
		if data.corpus_list.len() == 1 {
		    for (k, _) in data.corpus_list {
			cfg.default_corpus = k
		    }
		}
		confy::store("keynergy", cfg).unwrap();
	    }
	    CorpusCommand::Default => {
                corpus::default(&mut data, &mut cfg);
                data.save(&cfg);
	    }
	    CorpusCommand::Remove => {
                corpus::remove(&mut data);
                data.save(&cfg);
	    }
        },
	Commands::LoadMetrics => {
	    load_metrics::load_metrics(&data, &cfg);
	}
    }
}
