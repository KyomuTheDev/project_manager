use std::path::Path;

use crate::logging;

use super::utils;

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\python\\{}", name);	
	let project_directory = Path::new(&project_path);

	if !utils::is_initialized() {
		logging::error("Project not initialized. Please run `project init` first.");
		return;
	}

	if project_directory.exists() {
		logging::error("This project already exists!");
		return;
	}

	utils::create(&project_directory.to_path_buf());

	logging::info(&format!("Project {} created successfully!", name));

	utils::open(&project_path);
}