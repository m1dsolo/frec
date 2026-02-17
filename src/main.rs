mod algorithm;
mod commands;
mod db;

use clap::{Parser, Subcommand};
use db::Store;
use std::io;

#[derive(Parser)]
#[command(name = "frec")]
#[command(about = "A frecency-based tool for managing file weights", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(long = "table", default_value = "default")]
        table: String,
        path: String,
    },
    Query {
        #[arg(long = "table", default_value = "default")]
        table: String,
        #[arg(short, long, default_value = "false")]
        list: bool,
        #[arg(short, long, default_value = "false")]
        score: bool,
        keyword: Option<String>,
    },
    Init {
        #[arg(value_parser = ["bash", "zsh"])]
        shell: String,
    },
    Install,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { table, path } => {
            let store = Store::new()?;
            commands::add(&store, &table, &path)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }
        Commands::Query {
            table,
            list: _,
            score,
            keyword,
        } => {
            let store = Store::new()?;
            let results = commands::query(&store, &table, keyword.as_deref(), score)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            for r in results {
                if score {
                    println!("{} {}", r.score, r.path);
                } else {
                    println!("{}", r.path);
                }
            }
        }
        Commands::Init { shell } => {
            println!("{}", commands::init_shell(&shell));
        }
        Commands::Install => {
            commands::install()?;
        }
    }

    Ok(())
}
