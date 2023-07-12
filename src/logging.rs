#![allow(dead_code)]

use colored::Colorize;

pub fn log_warning(warning: &str) {
	println!("{}", warning.bright_yellow());
}

pub fn log_error(error: &str) {
	println!("{}", error.bright_red());
}

pub fn log_info(info: &str) {
	println!("{}", info.cyan());
}