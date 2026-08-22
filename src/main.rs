use std::path::Path;

use clearscreen::clear;
use create_java_web_willian_rs::builder::filesystem::create_project_structure;
use dialoguer::{Input, theme::ColorfulTheme};
use regex::Regex;

fn main() {
    clear().expect("Fail to clear screen");
    let java_docker_project_name_pattern = Regex::new(r"^[a-z0-9]([a-z0-9_-]*[a-z0-9])?$")
        .expect("Invalid regex pattern for Java/Docker project name");

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write Java project name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if java_docker_project_name_pattern.is_match(input) {
                Ok(())
            } else {
                Err("Invalid or unconventional name! Must match Java/Docker requirements")
            }
        })
        .interact_text()
        .expect("Failed to get project name");

    let project_path = Path::new(&project_name);

    if let Err(err) = create_project_structure(project_path) {
        eprintln!("Error creating the folder structure: {err}");
        std::process::exit(1);
    }

    clear().expect("Fail to clear screen");
    println!("Project name: {project_name}");

    #[cfg(windows)]
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("pause")
        .status();
}
