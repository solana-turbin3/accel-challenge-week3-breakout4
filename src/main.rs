use clap::{Parser, Subcommand};
mod commands;
mod index;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Branch { name: Option<String> }, // if no name list all branches else create one with name provided
    Switch { name: String },         // switch HEAD to the given branch
    Add { paths: Vec<String> },
    Commit { message: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init()?,
        Commands::Branch { name } => commands::branch::branch(name)?,
        Commands::Switch { name } => commands::switch::switch(name)?,
        Commands::Add { paths } => commands::add::add(paths)?,
        Commands::Commit { message } => commands::commit::commit(&message)?,
    }

    Ok(())
}
