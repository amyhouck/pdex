mod data;
mod lookup;

use clap::{Parser, Subcommand};

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
    }
}

fn main() {
    let cli = PDex::parse();
    let db = rusqlite::Connection::open("pdex.db").unwrap();
    
    match cli.command {
        Commands::Lookup { name, id } => lookup::lookup(&db, name, id),
    }
    
}
