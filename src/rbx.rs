use std::process::Command;
use std::path::Path;
use std::fs;
use json;
use json::object;
use crate::logging;

use super::utils;

fn edit_tree(path: &String, name: &String) {
	let result = fs::read_to_string(path).unwrap();

	let mut obj = json::parse(&result).unwrap();
	obj["name"] = json::JsonValue::String(name.to_string());

	obj["tree"]["ServerScriptService"]["server"] = obj["tree"]["ServerScriptService"]["Server"].clone();
	obj["tree"]["ReplicatedStorage"]["core"] = obj["tree"]["ReplicatedStorage"]["Common"].clone();
	obj["tree"]["StarterPlayer"]["StarterPlayerScripts"]["client"] = obj["tree"]["StarterPlayer"]["StarterPlayerScripts"]["Client"].clone();

	obj["tree"]["HttpService"] = object! {
		"$className": "HttpService",
		"$properties": object! {
			"HttpEnabled": true,
		},
	};

	obj["tree"]["StarterPlayer"]["StartPlayerScripts"].remove("Client");
	obj["tree"]["StarterPlayer"]["StartPlayerScripts"].remove("Server");
	obj["tree"]["StarterPlayer"]["StartPlayerScripts"].remove("Common");
	obj["tree"]["Workspace"].remove("Baseplate");

	match fs::write(path, json::stringify_pretty(obj, 4)) {
		Ok(_) => logging::info("Successfully edited tree"),
		Err(e) => logging::error(&format!("Failed to edit tree with error: {}", e.to_string())),
	};
}

pub fn new(name: &str) {
	let project_path = format!("C:\\projects\\rbx\\{}", name);
	let dir = Path::new(&project_path);

	if !utils::is_initialized() {
		logging::warning("Project not initialized. Please run `project init` first.");
		return;
	}

	if dir.exists() {
		logging::warning("This project already exists!");
		return;
	}

	let result = Command::new("cmd")
		.current_dir(&"C:\\projects\\rbx")
		.arg("/C")
		.arg("rojo")
		.arg("init")
		.arg(&name)
		.status();

	match result {
		Ok(_) => logging::info("Rojo successfully initialized!"),
		Err(err) => {
			logging::error(&format!("Failed to initialize Rojo with error: {}", err.to_string()));

			panic!("Rojo failed to initialize with error: {}", err.to_string())
		},
	}

	let mut json_file = project_path.clone();
	json_file.push_str("\\default.project.json");

	edit_tree(&json_file, &String::from(name));

	logging::info(&format!("Project {} created successfully!", name));

	utils::open(&project_path);
}