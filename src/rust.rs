use std::process::Command;
use std::path::Path;
use colored::Colorize;

use crate::logging;

use super::utils;

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\rust\\{}", name);	
	let dir = Path::new(&project_path);

	if !utils::is_initialized() {
		logging::info("Project not initialized. Please run `project init` first.");
		return;
	}

	if dir.exists() {
		logging::error("This project already exists!");
		return;
	}

	let result = Command::new("cmd")
		.current_dir(&"C:\\projects\\rust")
		.arg("/C")
		.arg("cargo")
		.arg("new")
		.arg(&name)
		.status();

	match result {
		Ok(_) => logging::info("Rust project successfully initialized!"),
		Err(e) => {
			logging::error(&format!("Cargo failed to create rust project with error: {}", e));

			panic!("{}", format!("Cargo failed to create rust project with error: {}", e).bright_red())
		}
	}

	logging::info(&format!("Project {} created successfully!", name));

	utils::open(&project_path);
}