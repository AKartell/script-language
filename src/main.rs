use std::{fs, path::PathBuf};
use clap::{command, Subcommand};
use lexer::{Lexer, Token};

mod lexer;
mod parse;
//mod evaluate;
#[derive(clap::Parser, Debug)]
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
    
    println!("{}", parse::Parser::new("let asd = asd").parse());
    return;
    let args = <Args as clap::Parser>::parse();
    
    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(filename).expect("Opening file failed!");
            for token in Lexer::new(&file_contents) {
                println!("{:?}", token);
            }
            
        },
        _ => println!("Semmi!")
    }
}