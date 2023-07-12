#![allow(unreachable_patterns)]

use clap::Parser;
use std::path::{Path, PathBuf};

mod logging;
mod args;
mod rbxts;
mod rbx;
mod rust;
mod python;
mod utils;

use args::ProjectManagerCommands;

struct Paths {
	rust: PathBuf,
	rbxts: PathBuf,
	rbx: PathBuf,
	python: PathBuf,
	project: PathBuf,
}

fn get_paths() -> Paths {
	return Paths {
		project: Path::new("C:\\projects").to_path_buf(),
		rust: Path::new("C:\\projects\\rust").to_path_buf(),
		rbxts: Path::new("C:\\projects\\rbxts").to_path_buf(),
		rbx: Path::new("C:\\projects\\rbx").to_path_buf(),
		python: Path::new("C:\\projects\\python").to_path_buf(),
	}
}

fn init() {
	let paths = get_paths();

	utils::create(&paths.project);
	utils::create(&paths.rust);
	utils::create(&paths.rbxts);
	utils::create(&paths.rbx);
	utils::create(&paths.python);

	logging::info("Project initialized!");
}

// fun comment

fn main() {
	logging::info("Running Project manager v3.0.0");

	let args = args::ProjectManagerArgs::parse();

	let command = args.command;

	match &command {
		ProjectManagerCommands::Init {} => init(),
		ProjectManagerCommands::Rename { project_type, name, new_name } => utils::rename(&project_type, &name, &new_name),
		ProjectManagerCommands::Delete { project_type, name } => utils::delete(&format!("C:\\projects\\{}\\{}", &project_type, &name)),
		ProjectManagerCommands::Open { project_type, name } => utils::open(&format!("C:\\projects\\{}\\{}", &project_type, &name)),
		ProjectManagerCommands::List { project_type } => {
			match &project_type {
				Some(t) => {
					let formatted = format!("C:\\projects\\{}\\", &t);
					let path = Path::new(&formatted);
				
					logging::info(&format!("{}:", t));
					// println!("{}", format!("{}:", t).cyan());
					utils::list(utils::ListFunction::Specific, Some(&path.to_path_buf()), Some(&String::from("  ")) );
				},
				None => utils::list(utils::ListFunction::All, None, None),
			}

			println!("\n");
		}
		ProjectManagerCommands::New { project_type, name } => {
			match project_type.as_str() {
				"rbx" => rbx::new(name),
				"rbxts" => rbxts::new(name),
				"rust" => rust::new(name),
				"python" => python::new(name),
				_ => {
					logging::error(&format!("Invalid project type: {}", project_type));
				}
			};
		},
		ProjectManagerCommands::Clone { project_type, name, new_name } => {
			match utils::clone(project_type, name, new_name) {
				Ok(_) => logging::info(&format!("{}", "Project cloned!")),
				Err(e) => {
					logging::error(&format!("Failed to clone project because: {}", e));
				}
			}
		},
		_ => {
			logging::error(&format!("Invalid command: {:?}", command));
		}
	}
}
