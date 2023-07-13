use std::process::Command;
use std::path::Path;
use colored::Colorize;

use crate::log;

use super::utils;

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\in_progress\\rust\\{}", name);	
	let dir = Path::new(&project_path);

	if !utils::is_initialized() {
		log::info("Project not initialized. Please run `project init` first.");
		return;
	}

	if dir.exists() {
		log::error("This project already exists!");
		return;
	}

	let result = Command::new("cmd")
		.current_dir(&"C:\\projects\\in_progress\\rust")
		.arg("/C")
		.arg("cargo")
		.arg("new")
		.arg(&name)
		.status();

	match result {
		Ok(_) => log::info("Rust project successfully initialized!"),
		Err(e) => {
			log::error(&format!("Cargo failed to create rust project with error: {}", e));

			panic!("{}", format!("Cargo failed to create rust project with error: {}", e).bright_red())
		}
	}

	log::info(&format!("Project {} created successfully!", name));

	utils::open(&project_path);
}