use std::path::Path;

use clearscreen::clear;
use create_java_web_willian_rs::builder::{
    filesystem::create_project_structure,
    generator::{
        download_mysql_connector, generate_build_file, generate_context_file, generate_css_file,
        generate_docker_files, generate_env_files, generate_gitignore_file, generate_index_file,
        generate_license_file, generate_logo_file, generate_readme_file, generate_web_xml,
    },
};
use dialoguer::{Confirm, Input, Password, theme::ColorfulTheme};
use regex::Regex;

fn main() {
    clear().expect("Fail to clear screen");
    let java_docker_project_name_pattern = Regex::new(r"^[a-z0-9]([a-z0-9_]*[a-z0-9])?$")
        .expect("Invalid regex pattern for Java/Docker/MySQL project name");

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write Java project name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if java_docker_project_name_pattern.is_match(input) {
                Ok(())
            } else {
                Err("Invalid or unconventional name! Must match Java/Docker/MySQL requirements")
            }
        })
        .interact_text()
        .expect("Failed to get project name");

    let project_path = Path::new(&project_name);

    let use_mysql = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to include MySQL support in the project?")
        .default(true)
        .interact()
        .expect("Failed to read MySQL option");

    let app_port: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Application HTTP port")
        .default("8081".to_string())
        .interact_text()
        .expect("Failed to read application port");

    let (db_user, db_pass, db_name, db_port) = if use_mysql {
        let user = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Database username")
            .default("root".to_string())
            .interact_text()
            .expect("Failed to read database user");

        let input_pass = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Database password")
            .allow_empty_password(true)
            .interact()
            .expect("Failed to read database password");

        let pass = if input_pass.is_empty() {
            "123".to_string()
        } else {
            input_pass
        };

        let name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Database name")
            .default(format!("{project_name}_db"))
            .interact_text()
            .expect("Failed to read database name");

        let port = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Database port")
            .default("3306".to_string())
            .interact_text()
            .expect("Failed to read database port");

        (user, pass, name, port)
    } else {
        (
            "root".to_string(),
            "123".to_string(),
            format!("{project_name}_db"),
            "3306".to_string(),
        )
    };

    if let Err(err) = create_project_structure(project_path) {
        eprintln!("Error creating the folder structure: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_build_file(project_path, &project_name) {
        eprintln!("Error generating build.xml file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_context_file(project_path, &db_name, &db_user, &db_pass) {
        eprintln!("Error generating context.xml file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_web_xml(project_path) {
        eprintln!("Error generating web.xml file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_index_file(project_path, &project_name, &db_name) {
        eprintln!("Error generating index.jsp file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_css_file(project_path) {
        eprintln!("Error generating style.css file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_logo_file(project_path) {
        eprintln!("Error generating logo.png file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_readme_file(project_path, &project_name) {
        eprintln!("Error generating README file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_license_file(project_path) {
        eprintln!("Error generating LICENSE file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_gitignore_file(project_path) {
        eprintln!("Error generating .gitignore file: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_env_files(
        project_path,
        &project_name,
        &db_name,
        &db_user,
        &db_pass,
        &app_port,
        &db_port,
    ) {
        eprintln!("Error generating .env files: {err}");
        std::process::exit(1);
    }

    if let Err(err) = generate_docker_files(project_path) {
        eprintln!("Error generating Docker files: {err}");
        std::process::exit(1);
    }

    if use_mysql && let Err(err) = download_mysql_connector(project_path) {
        eprintln!("Error downloading MySQL connector: {err}");
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
