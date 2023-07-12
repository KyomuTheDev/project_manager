#![allow(dead_code)]

use colored::Colorize;

pub fn warning(warning: &str) {
	println!("{}", warning.bright_yellow());
}

pub fn error(error: &str) {
	println!("{}", error.bright_red());
}

pub fn info(info: &str) {
	println!("{}", info.bright_green());
}