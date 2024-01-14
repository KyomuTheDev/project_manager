use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::log;

fn get_project_json(name: &str) -> String {
    format!(
        "{{
		\"name\": \"{}\",
		\"tree\": {{
		  \"$className\": \"DataModel\",
	  
		  \"ReplicatedStorage\": {{
			\"core\": {{
			  \"$path\": \"src/shared\"
			}}
		  }},
	  
		  \"ServerScriptService\": {{
			\"server\": {{
			  \"$path\": \"src/server\"
			}}
		  }},
	  
		  \"StarterPlayer\": {{
			\"StarterPlayerScripts\": {{
			  \"client\": {{
				\"$path\": \"src/client\"
			  }}
			}}
		  }},
	  
		  \"Workspace\": {{
			\"$properties\": {{
			  \"FilteringEnabled\": true
			}}
		  }},

		  \"SoundService\": {{
			\"$properties\": {{
			  \"RespectFilteringEnabled\": true
			}}
		  }}
		}}
	  }}",
        name
    )
}

fn edit_tree(path: &PathBuf, name: &String) {
    match fs::write(path, get_project_json(name)) {
        Ok(_) => log::info("Successfully edited tree"),
        Err(e) => log::error(&format!(
            "Failed to edit tree with error: {}",
            e.to_string()
        )),
    };
}

pub fn new(dir: &PathBuf, name: &String) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cmd")
        .arg("/c")
        .arg("rojo")
        .arg("init")
        .current_dir(dir)
        .status()?;

    edit_tree(&dir.join("default.project.json"), name);

    return Ok(());
}
