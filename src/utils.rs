/// named utils bc I suck at naming things

use std::error::Error;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use walkdir::WalkDir;
use colored::Colorize;

use crate::log;

pub fn open(path: &String) {
	if !Path::new(path).exists() {
		log::error(&format!("Project `{}` does not exist", path));
		return;
	}

	Command::new("cmd")
		.current_dir(path)
		.arg("/C")
		.arg("code")
		.arg(".")
		.spawn()
		.expect("Could not open project");
}

pub fn create(path: &PathBuf) -> () {
	if path.exists() {
		log::error(&format!("Project `{}` already exists", path.display().to_string()));
		return;
	}

	match fs::create_dir(&path) {
		Ok(_) => log::info(&format!("Created project directory: {}", path.display().to_string())),
		Err(e) => log::error(&format!("Failed to create project directory `{}` because {}", path.display().to_string(), e.to_string())),
	}
}

pub fn delete(path: &String) -> () {
	if !Path::new(path).exists() {
		log::warning(&format!("Project `{}` does not exist", path));
		return;
	}

	match fs::remove_dir_all(path) {
		Ok(_) => log::info(&format!("Deleted project directory: {}", path)),
		Err(e) => log::error(&format!("Failed to delete project directory `{}` because {}", path, e.to_string())),
	}
}
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ListFunction {
	All,
	Specific,
}

fn color_white(s: &str) -> () {
	println!("{}", s.bright_white().to_string())
}

fn color_pink(s: &str) -> () {
	println!("{}", s.truecolor(245, 169, 184).to_string())
}

fn color_blue(s: &str) -> () {
	println!("{}", s.truecolor(91, 206, 250).to_string())
}

#[allow(dead_code)]
pub fn list(func: ListFunction, path: Option<&PathBuf>, prefix: Option<&String>) {
	if func == ListFunction::All {
		color_blue("\nProjects:");

		for dir_entry in fs::read_dir("C:\\projects\\in_progress").unwrap() {
			let project_path = dir_entry.unwrap().path();
			color_pink(&format!("  {}", project_path.display().to_string()));

			list(ListFunction::Specific, Some(&project_path), Some(&String::from("    ")));
		}
	} else if func == ListFunction::Specific {
		if !path.unwrap().exists() {
			log::error(&format!("Project directory `{}` does not exist", path.unwrap().display().to_string()));
			return;
		}

		for dir_entry in fs::read_dir(path.unwrap()).unwrap() {
			let project_path = dir_entry.unwrap().path();

			color_white(&format!("{}{}", prefix.unwrap_or(&String::from("  ")), project_path.display().to_string()));
		}
	}
}

pub fn rename(t: &String, name: &String, new_name: &String) -> () {
	if !Path::new(&format!("C:\\projects\\in_progress\\{}\\{}", t, name)).exists() {
		log::error(&format!("Could not find project `{}` to rename", name));
		return;
	}
	if Path::new(&format!("C:\\projects\\in_progress\\{}\\{}", t, new_name)).exists() {
		log::error(&format!("Project `{}` already exists inside {}", new_name, t));
		return;
	}

	match fs::rename(&format!("C:\\projects\\in_progress\\{}\\{}", t, name), &format!("C:\\projects\\in_progress\\{}\\{}", t, new_name)) {
		Ok(_) => log::info(&format!("Renamed {} to {}", name, new_name)),
		Err(e) => log::error(&format!("Failed to rename {} to {} because {}", name, new_name, e.to_string())),
	}
}

pub fn clone(t: &String, name: &String, new_name: &String) -> Result<(), Box<dyn Error>> {
	let in_dir = PathBuf::from(&format!("C:\\projects\\in_progress\\{}\\{}", t, name));
	let out_dir = PathBuf::from(&format!("C:\\projects\\in_progress\\{}\\{}", t, new_name));

	if out_dir.exists() {
		log::error(&format!("Project `{}` already exists inside {}", new_name, t));
		return Err("Could not find project to clone".into());
	}

	if !in_dir.exists() {
		log::error(&format!("Could not find project `{}` to clone", name));
		return Err("Could not find project to clone".into());
	}

	for entry in WalkDir::new(&in_dir).into_iter() {
		let entry = entry?;

		let from = entry.path();
		let to = out_dir.join(from.strip_prefix(&in_dir)?);

		log::info(&format!("Cloning {}", from.display().to_string()));

		if entry.file_type().is_dir() {
			if let Err(e) = fs::create_dir(&to) {
				match e.kind() {
					std::io::ErrorKind::AlreadyExists => {}
					_ => return Err(e.into()),
				}
			}
		} else if entry.file_type().is_file() {
			fs::copy(from, to)?;
		} else {
			log::warning(&format!("Ignored symlink {}", from.display().to_string()));
		}
	}

	Ok(())
}

pub fn is_initialized() -> bool {
	return Path::new("C:\\projects").exists();
}
