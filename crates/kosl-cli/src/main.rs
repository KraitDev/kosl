use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(
    name = "kosl",
    about = "Krait Object Serialization Language CLI",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a KOSL file and print its internal AST representation
    Parse { file: String },
    /// Transpile a KOSL file to TOML (e.g. Cargo.kosl -> Cargo.toml)
    Transpile {
        file: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Format a KOSL file and output to stdout or overwrite in-place
    Format {
        file: String,
        #[arg(short, long)]
        write: bool,
    },
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
        Commands::Transpile { file, output } => {
            let content = fs::read_to_string(file)?;
            let mut parser = kosl_parser::Parser::new(&content);
            let ast = parser.parse()?;

            let toml = kosl_transpiler::kosl_to_toml(&ast)?;
            let out_path = output
                .clone()
                .unwrap_or_else(|| file.replace(".kosl", ".toml"));
            fs::write(&out_path, toml)?;
            println!("✅ Transpiled {} to {}", file, out_path);
        }
        Commands::Format { file, write } => {
            let content = fs::read_to_string(file)?;
            let mut parser = kosl_parser::Parser::new(&content);
            let ast = parser.parse()?;

            let formatted = kosl_formatter::format(&ast)?;
            if *write {
                fs::write(file, &formatted)?;
                println!("Formatted {}", file);
            } else {
                print!("{}", formatted);
            }
        }
    }
    Ok(())
}
