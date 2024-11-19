use clap::{command, Subcommand};
use evaluate::Evaluator;
use lexer::{Lexer, Token};
use std::{fs, path::PathBuf};

use std::io::{self, Write};
mod evaluate;
mod lexer;
mod parse;
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
    //let file_contents = fs::read_to_string(r#"C:\Users\tothk\Documents\Rust\script-language\test.simp"#).expect("Opening file failed!");
    let mut eval = Evaluator::new();
    loop {
        let mut input = String::new();

        print!(">>> "); // print the prompt
        io::stdout().flush().expect("Failed to flush stdout"); // flush to ensure it appears immediately

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let wasd = parse::Parser::new(&input).parse();
        if let Some(value) = eval.evaluate(wasd) {
            println!("{}", value);
        }
        
    }
    

    
    
    
    return;
    let args = <Args as clap::Parser>::parse();

    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(filename).expect("Opening file failed!");
            for token in Lexer::new(&file_contents) {
                println!("{:?}", token);
            }
        }
        _ => println!("Semmi!"),
    }
}
