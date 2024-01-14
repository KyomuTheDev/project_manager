use std::{path::PathBuf, process::Command};

pub fn new(dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cmd")
        .arg("/c")
        .arg("cargo")
        .arg("init")
        .current_dir(dir)
        .status()?;

    Ok(())
}
