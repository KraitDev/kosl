use clap::{Parser, Subcommand};
use std::fs;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "kosl", about = "Krait Object Serialization Language CLI", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a KOSL file and print its internal AST representation
    Parse { file: String },
    /// Transpile a KOSL file to TOML (e.g. Cargo.kosl -> Cargo.toml)
    Transpile { file: String, #[arg(short, long)] output: Option<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let content = fs::read_to_string(file)?;
            let mut parser = kosl_parser::Parser::new(&content);
            let ast = parser.parse()?;
            println!("{:#?}", ast);
        }
        Commands::Commands::Transpile { file, output } => {
            let content = fs::read_to_string(file)?;
            let mut parser = kosl_parser::Parser::new(&content);
            let ast = parser.parse()?;
            
            let toml = kosl_transpiler::kosl_to_toml(&ast)?;
            let out_path = output.clone().unwrap_or_else(|| file.replace(".kosl", ".toml"));
            fs::write(&out_path, toml)?;
            println!("✅ Transpiled {} to {}", file, out_path);
        }
    }
    Ok(())
}