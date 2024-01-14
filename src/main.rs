#![allow(unreachable_patterns)]

use args::{Arguments, Commands, Kind, TagCommand};
use utils::{ensure_exists, check_init, get_folder_path, project_exists, write_metadata, read_metadata, init_regkey, check_exists};

use clap::Parser;
use serde_json::{json, Value};
use walkdir::WalkDir;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{prelude::*, Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

mod args;
mod error;
mod log;
mod rbx;
mod rbxts;
mod rust;
mod utils;

type MainResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> MainResult {
    let args = Arguments::parse();

    match args.command {
        Commands::Init { folder_path } => init(folder_path),
        Commands::New { name, kind, tags } => new(name, kind, tags),
        Commands::Delete { name } => delete(name),
        Commands::List { tags } => list(tags),
        Commands::Open { name } => open(name),
        Commands::Rename { name, new_name } => rename(name, new_name),
        Commands::Clone { name } => clone(name),
        Commands::Complete { name } => complete(name),
        Commands::Tag { tag_command } => tag(tag_command),
    }
}

fn init(folder_path: Option<PathBuf>) -> MainResult {
    let mut path = match folder_path {
        Some(p) => p,
        None => PathBuf::from("C:\\projects"),
    };

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let key = init_regkey()?;

    key.set_value("folder_path", &path.to_string_lossy().to_string())?;

    path.push("metadata.json");

    File::create(&path)?;

    let v_json: Value = json!({
        "projects": []
    });

    let json = serde_json::to_string_pretty(&v_json)?;

    let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;

    file.write(&json.as_bytes())?;

    Ok(())
}

fn new(name: String, kind: Kind, tags: Option<Vec<String>>) -> MainResult {
    check_init()?;
    ensure_exists(&name)?;

    let mut metadata = read_metadata()?;

    metadata.projects.push(utils::Project {
        name: name.clone(),
        kind: kind.clone(),
        tags: tags.unwrap_or(vec![]),
        path: get_folder_path().join("projects").join(&name),
    });

    write_metadata(metadata)?;
    let project_path = get_folder_path().join("projects").join(&name);
    fs::create_dir(&project_path)?;

    match kind {
        Kind::Python | Kind::None => {}
        Kind::Rbx => {
			rbx::new(&project_path, &name)?;
		},
        Kind::Rbxts { template } => {
			rbxts::new(&project_path, &name, template)?;
		},
        Kind::Rust => {
			rust::new(&project_path)?;
		},
    };

	open(name)?;

    Ok(())
}

fn delete(name: String) -> MainResult {
    check_init()?;

    println!("Are you sure you want to delete {}? y/n", name);

    let mut answer = String::new();

    io::stdin().read_line(&mut answer)?;

    answer = answer.to_lowercase();

    // I will never not be annoyed by windows using \r\n
    if answer == "n\r\n" || answer == "n\n" {
        println!("Process aborted");
        return Ok(());
    } else if answer != "y\r\n" || answer != "y\n" {
        println!("Invalid answer, process aborted");
        return Ok(());
    }

    let mut metadata = read_metadata()?;

    let mut count: usize = 0;
    for project in &mut metadata.projects {
        count += 1;
        if project.name == name {
            break;
        }
    }

    let project = metadata.projects.get(count - 1).unwrap();

    fs::remove_dir_all(&project.path)?;
    metadata.projects.swap_remove(count - 1);

    fs::write(
        get_folder_path().join("metadata.json"),
        serde_json::to_string_pretty(&metadata)?,
    )?;

    Ok(())
}

fn list(tags: Option<Vec<String>>) -> MainResult {
    check_init()?;

    let p_tags = match tags {
        Some(tags) => tags,
        None => vec![],
    };

    let metadata = read_metadata()?;

    for project in metadata.projects {
        let mut count: usize = 0;

        for tag in &p_tags {
            if !project.tags.contains(tag) {
                break;
            }

            count += 1;
        }

        if count == p_tags.len() {
            println!("{}", project.name);
        }
    }

    Ok(())
}

fn open(name: String) -> MainResult {
    check_exists(&name)?;
    project_exists(&name)?;

    Command::new("cmd")
        .current_dir(get_folder_path().join("projects").join(&name))
        .arg("/c")
        .arg("code")
        .arg(".")
        .spawn()
        .expect("Code failed to start");

    Ok(())
}

fn rename(name: String, new_name: String) -> MainResult {
    check_init()?;
    project_exists(&name)?;

    let current_path = get_folder_path().join("projects").join(&name);
    let new_path = get_folder_path().join("projects").join(&new_name);

    if new_path.exists() {
        return Err(Box::new(Error::new(
            ErrorKind::PermissionDenied,
            "Project already exists",
        )));
    }

    fs::rename(&current_path, &new_path)?;

    Ok(())
}

fn clone(name: String) -> MainResult {
    check_init()?;
    project_exists(&name)?;
    let current_path = get_folder_path().join("projects").join(&name);
    let clone_path = ensure_exists(&(name.clone() + "_clone"))?;
    for entry in WalkDir::new(&current_path) {
        let entry = entry?;

        let from = entry.path();
        let to = clone_path.join(entry.file_name());

        if entry.file_type().is_dir() {
            fs::create_dir(&to)?;
        } else if entry.file_type().is_file() {
            fs::copy(from, &to)?;
        } else {
            println!("Ignored symlink {}", from.display().to_string());
        }
    }
    Ok(())
}

fn complete(name: String) -> MainResult {
    check_init()?;
    let metadata = read_metadata()?;
    metadata.projects.into_iter().for_each(|mut project| {
        if project.name != name {
            return;
        }

        if !project.tags.contains(&"complete".to_string()) {
            project.tags.push("complete".to_string());
        }
    });
    Ok(())
}

fn tag(tag_command: TagCommand) -> MainResult {
    check_init()?;
    match tag_command {
        TagCommand::Add { name, tag } => {
            let mut metadata = read_metadata()?;

            metadata.projects.iter_mut().for_each(|project| {
                if project.name != name {
                    return;
                }
                project.tags.push(tag.clone());
            });

            write_metadata(metadata)
        }

        TagCommand::Remove { name, tag } => {
            let mut metadata = read_metadata()?;

            metadata.projects.iter_mut().for_each(|project| {
                if project.name != name {
                    return;
                }
                project.tags.retain(|t| t != &tag);
            });

            write_metadata(metadata)
        }
    }
}
