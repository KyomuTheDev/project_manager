use clap::{
	Parser,
	Subcommand,
};

#[derive(Parser, Debug)]
pub struct ProjectManagerArgs {
	/// What to do
	#[clap(subcommand)]
	pub command: ProjectManagerCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProjectManagerCommands {
	/// Create a new project
	New { 
		/// The type of project to create
		project_type: String, 
		/// The name of the project
		name: String 
	},

	/// Delete a project
	Delete { 
		/// The type of project to delete
		project_type: String, 
		/// The name of the project
		name: String 
	},

	/// List projects with a optional project type
	List { 
		/// The type of projects to list
		project_type: Option<String> 
	},

	/// Open a project
	Open { 
		/// The type of project to open
		project_type: String, 
		/// The name of the project
		name: String 
	},

	/// Rename a project
	Rename { 
		/// The type of project to rename
		project_type: String, 
		/// The name of the project
		name: String,
		/// The new name of the project
		new_name: String
	},

	/// Clone a project
	Clone { 
		/// The type of project to clone
		project_type: String, 
		/// The name of the project
		name: String,
		/// The new name of the project
		new_name: String
	},

	/// Initialize project manager
	Init {},
}
