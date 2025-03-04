mod data;
mod lookup;

use clap::{Parser, Subcommand};
use anyhow::{Context, Result};

const LATEST_GENERATION: i64 = 3;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct PDex {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Lookup {
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,
        
        #[arg(short, long, value_name = "ID")]
        id: Option<i64>,
        
        #[arg(short, long, value_name = "FORM")]
        form: Option<String>,
        
        #[arg(short, long, value_name = "GENERATION")]
        generation: Option<i64>,
    }
}

fn main() -> Result<()> {
    let cli = PDex::parse();
    let db = rusqlite::Connection::open("pdex.db").unwrap();
    
    match cli.command {
        Commands::Lookup { name, id , generation, form} => lookup::lookup(&db, name, id, generation, form).context("Error occurred in lookup command.")?,
    }
    
    Ok(())
}
