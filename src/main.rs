use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Branch { name: Option<String> }, // if no name list all branches else create one with name provided
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init()?,
        Commands::Branch { name } => commands::branch::branch(name)?,
    }

    Ok(())
}
