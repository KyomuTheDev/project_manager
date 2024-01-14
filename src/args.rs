use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        folder_path: Option<PathBuf>,
    },
    New {
        name: String,
		#[command(subcommand)]
        kind: Kind,
        tags: Option<Vec<String>>,
    },
    Delete {
        name: String,
    },
    List {
        tags: Option<Vec<String>>,
    },
    Open {
        name: String,
    },
    Rename {
        name: String,
        new_name: String,
    },
    Clone {
        name: String,
    },
    Complete {
        name: String,
    },
    Tag {
        #[command(subcommand)]
        tag_command: TagCommand,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum TagCommand {
    Add { name: String, tag: String },
    Remove { name: String, tag: String },
}

#[derive(Debug, Clone, Subcommand, Deserialize, Serialize)]
pub enum Kind {
    Python,
    Rbx,
	Rust,
	None,
    Rbxts { 
		template: Option<String>,
	},
}
