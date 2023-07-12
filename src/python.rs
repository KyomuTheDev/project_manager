use std::path::Path;

use crate::log;

use super::utils;

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\python\\{}", name);	
	let project_directory = Path::new(&project_path);

	if !utils::is_initialized() {
		log::error("Project not initialized. Please run `project init` first.");
		return;
	}

	if project_directory.exists() {
		log::error("This project already exists!");
		return;
	}

	utils::create(&project_directory.to_path_buf());

	log::info(&format!("Project {} created successfully!", name));

	utils::open(&project_path);
}