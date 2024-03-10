use project::Project;
use std::{fs, path::PathBuf, process::Command};

use args::Args;
use clap::Parser;
use dialoguer::{Confirm, Input, Select};

const PROJECTS_PATH: &str = "./projects.json";

mod args;
mod project;

fn read_projects() -> Vec<Project> {
    let projects: Vec<Project> =
        serde_json::from_str(fs::read_to_string(PROJECTS_PATH).unwrap().as_str()).unwrap();

    projects
}

fn write_projects(projects: &Vec<Project>) {
    fs::write(PROJECTS_PATH, serde_json::to_string(projects).unwrap()).unwrap();
}

fn main() {
    let command = Args::parse();

    match command {
        Args::Open => open(),
        Args::Remove => remove(),
        Args::Add => add(),
        Args::Delete => delete(),
		Args::View => view(),
        _ => {}
    }
}

fn open() {
    let projects = read_projects();

    let selection = Select::new()
        .with_prompt("Select the project you want to open.")
        .items(&projects)
        .interact_opt()
        .unwrap();

    if selection.is_none() {
        return println!("Aborted.");
    }

    let selected_project = projects[selection.unwrap()].clone();

    let status = Command::new("cmd")
        .arg("/c")
        .args(["code", "."])
        .current_dir(selected_project.path)
        .status();

    match status {
        Ok(_) => {}
        Err(e) => println!("An error occured while opening the project: {}", e),
    }
}

fn add() {
    let s_path: String = Input::new()
        .with_prompt(
            "Enter the path to the project(Relative is allowed, it will be converted to absolute).",
        )
        .interact_text()
        .unwrap();

    let path = PathBuf::from(&s_path).canonicalize().unwrap();

    let mut projects = read_projects();

    for project in &projects {
        if project.path == path {
            println!("This project already exists: {}", project.name);
            return;
        }
    }

    let name: String = Input::new()
        .with_prompt("Enter the name of the project")
        .interact_text()
        .unwrap();

    let tags = {
        let str_tags: String = Input::new()
            .with_prompt("Enter the tags for this project, seperated by comma's")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let mut tags: Vec<String> = vec![];

        for tag in str_tags.split(",") {
            tags.push(tag.trim().to_string())
        }

        tags
    };

    projects.push(Project { name, path, tags });

    write_projects(&projects)
}

fn remove() {
    let mut projects = read_projects();

    let project = Select::new()
        .with_prompt("Select the project you want to remove.")
        .items(&projects)
        .interact_opt()
        .unwrap();

    if project.is_none() {
        return println!("Aborted");
    }

    projects.swap_remove(project.unwrap());

    write_projects(&projects);
}

fn delete() {
    let mut projects = read_projects();

    let project = Select::new()
        .with_prompt("Select the project you want to delete")
        .items(&projects)
        .interact_opt()
        .unwrap();

    if project.is_none() {
        return println!("Aborted.");
    }

    if !Confirm::new()
        .with_prompt("Are you sure you want to delete this project?")
        .interact()
        .unwrap()
    {
        return println!("Aborted.");
    }

    let removed = projects.swap_remove(project.unwrap());

    fs::remove_dir_all(removed.path).unwrap();

    write_projects(&projects);
}

fn view() {
	let tags: String = Input::new()
		.with_prompt("Enter the tags to search, seperated by comma's.")
		.allow_empty(true)
		.interact_text()
		.unwrap();
	
	if tags.len() == 0 {
		view_unfiltered()
	} else {
		view_filtered(tags);
	}
}

fn view_filtered(s_tags: String) {
	let mut tags = vec![];

	for tag in s_tags.split(",") {
		tags.push(tag.trim().to_string())
	}

	for project in read_projects() {
		let mut has_tags = true;

		for tag in &tags {
			if !project.tags.contains(&tag) { has_tags = false; break; }
		}

		if !has_tags { continue; }

		println!("{}", project);
	}
}

fn view_unfiltered() {
	for project in read_projects() {
		println!("{}", project);
	}
}