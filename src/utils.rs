use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use crate::args::Kind;
use crate::error::ProjectError;

use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub kind: Kind,
    pub tags: Vec<String>,
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub projects: Vec<Project>,
}

pub fn init_regkey() -> Result<RegKey, Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key_path = Path::new("Software").join("ApolloPM");
    let (key, _) = hkcu.create_subkey(&key_path)?;

    Ok(key)
}

pub fn get_folder_path() -> PathBuf {
    let key = init_regkey().unwrap();
    let folder_path: String = key.get_value("folder_path").unwrap();

    PathBuf::from(folder_path)
}

pub fn read_metadata() -> Result<Metadata, Box<dyn std::error::Error>> {
    let json_value: Metadata = serde_json::from_value(serde_json::from_str(&fs::read_to_string(
        get_folder_path().join("metadata.json"),
    )?)?)?;

    Ok(json_value)
}

pub fn write_metadata(metadata: Metadata) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(
        get_folder_path().join("metadata.json"),
        serde_json::to_string_pretty(&metadata)?,
    )?;

    Ok(())
}

pub fn project_exists(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = read_metadata()?;

    for project in metadata.projects {
        if project.name == name {
            return Ok(());
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "Project not found",
    )))
}

pub fn is_initialized() -> bool {
    init_regkey().is_ok() & get_folder_path().exists()
}

pub fn ensure_exists(name: &String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut answer = name.clone();

    let path = get_folder_path().join("projects").join(&answer);

    if path.exists() {
        answer = loop {
            println!(
                "Project({}) already exists, please choose a different name",
                path.file_name()
                    .expect("Invalid path name")
                    .to_str()
                    .expect("I don't even know")
            );

            io::stdin().read_line(&mut answer)?;

            answer = answer.clone();

            let path = get_folder_path().join("projects").join(&answer);

            if path.exists() {
                continue;
            }

            break answer;
        }
    }

    return Ok(get_folder_path().join("projects").join(&answer));
}

pub fn check_exists(name: &String) -> Result<(), Box<dyn std::error::Error>> {
	if get_folder_path().join("projects").join(name).exists() {
		Ok(())
	} else {
		Err(Box::new(ProjectError::new(&format!("Project({}) does not exist", name))))
	}
}

pub fn check_init() -> Result<(), Box<dyn std::error::Error>> {
    if !is_initialized() {
        crate::log::warning("Project not initialized, would you like to initialize it? (y/n)");

        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        answer = answer.to_lowercase();

        if answer != "y\r\n" || answer != "y\n" {
            crate::log::warning("Process aborted");
            return Err(Box::new(crate::error::ProjectError::new("Process aborted")));
        }

        Command::new("cmd")
            .arg("/c")
            .arg("project")
            .arg("init")
            .spawn()?;
    }

    Ok(())
}
