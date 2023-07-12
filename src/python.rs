use std::path::Path;
use colored::Colorize;

use crate::logging;

use super::utils;

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\python\\{}", name);	
	let project_directory = Path::new(&project_path);

	if !utils::is_initialized() {
		logging::error(&format!("{}", "Project not initialized. Please run `project init` first."));
		return;
	}

	if project_directory.exists() {
		logging::error(&format!("{}", "This project already exists!"));
		return;
	}

	utils::create(&project_directory.to_path_buf());

	logging::info(&format!("{}", format!("Project {} created successfully!", name).bright_green()));

	utils::open(&project_path);
}