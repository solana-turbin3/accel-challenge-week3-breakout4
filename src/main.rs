#![allow(warnings)]
use clap::{Parser, Subcommand};

use crate::commands::log::log_commit;
mod commands;
mod error;
mod index;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Branch {
        name: Option<String>,
    },
    Switch {
        name: String,
    },
    Add {
        paths: Vec<String>,
    },
    Log,
    Commit {
        #[arg(short)]
        message: String,
    },
    Reset,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init()?,
        Commands::Branch { name } => commands::branch::branch(name)?,
        Commands::Switch { name } => commands::switch::switch(name)?,
        Commands::Add { paths } => commands::add::add(paths)?,
        Commands::Log => commands::log::log()?,
        Commands::Commit { message } => {
            let tree_hash = commands::write_tree::write_tree()?;
            let parent_hash = commands::commit_tree::get_parent()?;
            commands::commit_tree::commit_tree(&tree_hash, parent_hash, &message)?;
        }
        Commands::Reset => commands::reset::reset()?,
    }

    Ok(())
}
