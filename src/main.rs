#![allow(unreachable_patterns)]

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod log;
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
	blender: PathBuf,
	unity: PathBuf,
	project: PathBuf,
	completed: PathBuf,
	in_progress: PathBuf,
}

fn get_paths() -> Paths {
	return Paths {
		project: Path::new("C:\\projects").to_path_buf(),
		completed: Path::new("C:\\projects\\completed").to_path_buf(),
		in_progress: Path::new("C:\\projects\\in_progress").to_path_buf(),
		rust: Path::new("C:\\projects\\in_progress\\rust").to_path_buf(),
		rbxts: Path::new("C:\\projects\\in_progress\\rbxts").to_path_buf(),
		rbx: Path::new("C:\\projects\\in_progress\\rbx").to_path_buf(),
		python: Path::new("C:\\projects\\in_progress\\python").to_path_buf(),
		blender: Path::new("C:\\projects\\in_progress\\blender").to_path_buf(),
		unity: Path::new("C:\\projects\\in_progress\\unity").to_path_buf(),
	}
}

fn init() {
	let paths = get_paths();

	utils::create(&paths.project);
	utils::create(&paths.completed);
	utils::create(&paths.in_progress);
	utils::create(&paths.rust);
	utils::create(&paths.rbxts);
	utils::create(&paths.rbx);
	utils::create(&paths.python);
	utils::create(&paths.blender);
	utils::create(&paths.unity);

	log::info("Project initialized!");
}

// fun comment

fn main() {
	log::info("Running Project manager v3.0.0");

	let args = args::ProjectManagerArgs::parse();

	let command = args.command;

	match &command {
		ProjectManagerCommands::Init {} => init(),
		ProjectManagerCommands::Rename { project_type, name, new_name } => utils::rename(&project_type, &name, &new_name),
		ProjectManagerCommands::Delete { project_type, name } => utils::delete(&format!("C:\\projects\\in_progress\\{}\\{}", &project_type, &name)),
		ProjectManagerCommands::Open { project_type, name } => utils::open(&format!("C:\\projects\\in_progress\\{}\\{}", &project_type, &name)),
		ProjectManagerCommands::List { project_type } => {
			match &project_type {
				Some(t) => {
					let formatted = format!("C:\\projects\\in_progress\\{}\\", &t);
					let path = Path::new(&formatted);
				
					log::info(&format!("{}:", t));
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
					log::error(&format!("Invalid project type: {}", project_type));
				}
			};
			
			Command::new("cmd")
				.arg("/C")
				.current_dir(format!("C:\\projects\\in_progress\\{}\\{}", &project_type, &name))
				.arg("git")
				.arg("init");
		},
		ProjectManagerCommands::Clone { project_type, name, new_name } => {
			match utils::clone(project_type, name, new_name) {
				Ok(_) => log::info(&format!("{}", "Project cloned!")),
				Err(e) => {
					log::error(&format!("Failed to clone project because: {}", e));
				}
			}
		},
		ProjectManagerCommands::Completed { project_type, name } => {
			let path_str = format!("C:\\projects\\in_progress\\{}\\{}", &project_type, &name);
			let path = Path::new(&path_str);

			if !path.exists() {
				log::error(&format!("That project does not exist!"));
				return;
			}

			let completed_str = format!("C:\\projects\\completed\\{}_{}", &project_type, &name);
			let completed = Path::new(&completed_str);

			if completed.exists() {
				log::error("That project is already completed!");
				return;
			}

			match fs::rename(path, completed) {
				Ok(_) => log::info(&format!("{}", "Project completed!")),
				Err(e) => {
					log::error(&format!("Failed to complete project because: {}", e));
				}
			}
		},
		_ => {
			log::error(&format!("Invalid command: {:?}", command));
		}
	}
}
