use std::{fs, path::PathBuf};
use clap::{command, Parser, Subcommand};
use lexer::{Lexer, Token};

mod lexer;
mod parse;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Tokenize { filename: PathBuf },
    Parse { filename: PathBuf },
    Run { filename: PathBuf },
}
fn main() {
    let parser = parse::Parser::new("");
    let string = parser.parse();

    println!("{}", string);
    return;
    let args = Args::parse();
    
    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(filename).expect("Opening file failed!");
            for token in Lexer::new(&file_contents) {
                println!("{:?}", token);
            }
            
        },
        _ => todo!()
    }
}