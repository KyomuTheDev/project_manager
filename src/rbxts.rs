use std::process::Command;
use std::path::Path;
use std::fs;
use colored::Colorize;
use json;
use crate::log;

use super::utils;

fn edit_tree(path: &String, name: &String) {
	let result = fs::read_to_string(path).unwrap();

	let mut obj = json::parse(&result).unwrap();
	obj["name"] = json::JsonValue::String(String::from(name));

	obj["tree"]["ServerScriptService"]["server"] = obj["tree"]["ServerScriptService"]["TS"].clone();
	obj["tree"]["ReplicatedStorage"]["core"] = obj["tree"]["ReplicatedStorage"]["TS"].clone();
	obj["tree"]["StarterPlayer"]["StarterPlayerScripts"]["client"] = obj["tree"]["StarterPlayer"]["StarterPlayerScripts"]["TS"].clone();

	obj["tree"]["ServerScriptService"].remove("TS");
	obj["tree"]["ReplicatedStorage"].remove("TS");
	obj["tree"]["StarterPlayer"]["StarterPlayerScripts"].remove("TS");

	match fs::write(&path, json::stringify_pretty(obj, 4)) {
		Ok(_) => log::info("Successfully edited tree!"),
		Err(e) => log::error(&format!("Failed to edit tree with error: {}", e.to_string()))
	};
}

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\rbxts\\{}", name);
	let dir = Path::new(&project_path);

	if !utils::is_initialized() {
		log::warning("Project not initialized. Please run `project init` first.");
		return;
	}

	if dir.exists() {
		log::warning("This project already exists!");
		return;
	}

	utils::create(&dir.to_path_buf());

	let result = Command::new("cmd")
		.current_dir(&project_path)
		.arg("/C")
		.arg("rbxtsc")
		.arg("init")
		.arg("game")
		.status();

	match result {
		Ok(_) => log::info("rbxtsc successfully initialized!"),
		Err(err) => {
			log::error(&format!("rbxtsc failed to initialize with error: {}", err.to_string()));

			panic!("{}", format!("rbxtsc failed to initialize: {}", err).bright_red())
		},
	}

	let mut json_file = project_path.clone();
	json_file.push_str("\\default.project.json");

	edit_tree(&json_file, &String::from(name));

	log::info(&format!("Project {} created successfully!", name));

	utils::open(&project_path);
}