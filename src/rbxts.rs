use crate::log;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_project_json(name: &String) -> String {
    format!(
        "{{
		\"name\": \"{}\",
		\"globIgnorePaths\": [
			\"**/package.json\",
			\"**/tsconfig.json\"
		],
		\"tree\": {{
			\"$className\": \"DataModel\",
			\"ServerScriptService\": {{
				\"$className\": \"ServerScriptService\",
				\"server\": {{
					\"$path\": \"out/server\"
				}}
			}},
			\"ReplicatedStorage\": {{
				\"$className\": \"ReplicatedStorage\",
				\"rbxts_include\": {{
					\"$path\": \"include\",
					\"node_modules\": {{
						\"$className\": \"Folder\",
						\"@rbxts\": {{
							\"$path\": \"node_modules/@rbxts\"
						}}
					}}
				}},
				\"core\": {{
					\"$path\": \"out/shared\"
				}}
			}},
			\"StarterPlayer\": {{
				\"$className\": \"StarterPlayer\",
				\"StarterPlayerScripts\": {{
					\"$className\": \"StarterPlayerScripts\",
					\"client\": {{
						\"$path\": \"out/client\"
					}}
				}}
			}},
			\"Workspace\": {{
				\"$className\": \"Workspace\",
				\"$properties\": {{
					\"FilteringEnabled\": true
				}}
			}},
			\"HttpService\": {{
				\"$className\": \"HttpService\",
				\"$properties\": {{
					\"HttpEnabled\": true
				}}
			}},
			\"SoundService\": {{
				\"$className\": \"SoundService\",
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
    match fs::write(&path, get_project_json(name)) {
        Ok(_) => log::info("Successfully edited tree!"),
        Err(e) => log::error(&format!(
            "Failed to edit tree with error: {}",
            e.to_string()
        )),
    };
}

pub fn new(
    dir: &PathBuf,
    name: &String,
    template: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cmd")
        .arg("/c")
        .arg("rbxtsc")
        .arg("init")
        .arg(template.unwrap_or("game".to_string()))
        .current_dir(dir)
        .status()?;

    edit_tree(&dir.join("default.project.json"), name);

    Ok(())
}
