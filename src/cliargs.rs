use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input source file
    pub input: String,
    /// Output file
    #[arg(short, long, default_value = "a.out")]
    pub output: String,
    /// Run scanner but do not parse tokens
    #[arg(short, long)]
    pub lex: bool,
    /// Run parser but do not generate code
    #[arg(short, long)]
    pub parse: bool,
    /// Generate code
    #[arg(short, long)]
    pub codegen: bool,
}
