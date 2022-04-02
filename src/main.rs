use clap::{Parser, Subcommand};
use std::io::Read;
use std::path::PathBuf;

/// 4 in 5 sardines recommend this CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds your project
    Build {
        /// The relative path to a file
        #[clap(short, long, parse(from_os_str), value_name = "FILE")]
        file: Option<PathBuf>,
    },
}

fn main() {
    let cli = Args::parse();

    // load configuration file
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }
    // enable debug mode
    if cli.debug {
        println!("verbose: {:?}", cli.debug);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Build { file }) => {
            if let Some(file_path) = file.as_deref() {
                let mut file = std::fs::File::open(file_path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                print!("{}", contents);
            }
        }
        None => {}
    }
}
