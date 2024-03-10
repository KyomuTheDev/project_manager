use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "project manager")]
#[command(version = "1.0")]
#[command(about = "Manages coding projects")]
pub enum Args {
    New,            // Create a project from a template
    Add,            // Add a existing project to be managed
    Remove,         // Remove a existing project from the manager but do not delete it
    Delete,         // Delete a project, removing it from the manager and deleting it off the drive
    Open,           // Open a project in your perferred editor
    View,           // View managed projects
    AddTemplate,    // Add a project template, locally or through a url
    RemoveTemplate, // Remove a project template(Does not and will never delete the template itself)
    ViewTemplates,  // View available templates
    Settings,       // Open the managers settings
    Backup,			// Backup all projects
    Restore,		// Restore a specific project from a backup
}
