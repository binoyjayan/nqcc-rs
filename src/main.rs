use clap::Parser;
use std::fs;
use std::io;
use std::process;

mod cliargs;
mod scanner;

use scanner::token::TokenType;

fn main() {
    let args = cliargs::Args::parse();

    match run_file(&args.input) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

pub fn run_file(path: &String) -> io::Result<()> {
    let buf = fs::read_to_string(path)?;
    let mut scanner = scanner::Scanner::new(&buf);
    let mut errors = 0;

    loop {
        let tok = scanner.scan_token();

        match tok.ttype {
            TokenType::Error => {
                eprintln!("{}:{}: {}", path, tok.line, tok.lexeme);
                errors += 1;
            }
            TokenType::Eof => {
                println!("{:?}", tok);
                break;
            }
            _ => {
                println!("{:?}", tok);
            }
        }
    }
    if errors > 0 {
        eprintln!("{} error(s) found", errors);
        process::exit(65);
    }
    Ok(())
}
