use std::{
    fs::{self, File},
    io,
    path::Path,
};

const BUILD_XML: &str = include_str!("../../templates/build.xml");
const CONTEXT_XML: &str = include_str!("../../templates/context.xml");
const WEB_XML: &str = include_str!("../../templates/web.xml");

const INDEX_JSP: &str = include_str!("../../templates/index.jsp");
const STYLE_CSS: &str = include_str!("../../templates/style.css");

const README: &str = include_str!("../../templates/README.md");
const LICENSE: &str = include_str!("../../templates/LICENSE");
const GITIGNORE: &str = include_str!("../../templates/.gitignore");

const ENV_FILE: &str = include_str!("../../templates/env");
const ENV_EXAMPLE_FILE: &str = include_str!("../../templates/env.example");

const DOCKERFILE: &str = include_str!("../../templates/Dockerfile");
const DOCKER_COMPOSE: &str = include_str!("../../templates/docker-compose.yml");
const DOCKERIGNORE: &str = include_str!("../../templates/.dockerignore");

const MYSQL_CONNECTOR_URL: &str =
    "https://repo1.maven.org/maven2/com/mysql/mysql-connector-j/9.2.0/mysql-connector-j-9.2.0.jar";
const MYSQL_JAR_NAME: &str = "mysql-connector-j-9.2.0.jar";

pub fn generate_build_file(project_path: &Path, project_name: &str) -> io::Result<()> {
    let content = BUILD_XML.replace("project_name", project_name);

    let build_file_path = project_path.join("build.xml");

    fs::write(build_file_path, content)?;

    Ok(())
}

pub fn generate_context_file(
    project_path: &Path,
    project_name: &str,
    db_name: &str,
    db_user: &str,
    db_pass: &str,
) -> io::Result<()> {
    let content = CONTEXT_XML
        .replace("project_name", project_name)
        .replace("project_db_name", db_name)
        .replace("project_db_user", db_user)
        .replace("project_db_pass", db_pass);

    let context_file_path = project_path
        .join("web")
        .join("META-INF")
        .join("context.xml");

    fs::write(context_file_path, content)?;

    Ok(())
}

pub fn generate_web_xml(project_path: &Path) -> io::Result<()> {
    let web_xml_path = project_path.join("web").join("WEB-INF").join("web.xml");

    fs::write(web_xml_path, WEB_XML)?;

    Ok(())
}

pub fn generate_index_file(project_path: &Path, project_name: &str) -> io::Result<()> {
    let content = INDEX_JSP.replace("project_name", project_name);

    let index_file_path = project_path.join("web").join("index.jsp");

    fs::write(index_file_path, content)?;

    Ok(())
}

pub fn generate_css_file(project_path: &Path) -> io::Result<()> {
    let css_file_path = project_path
        .join("web")
        .join("assets")
        .join("css")
        .join("style.css");

    fs::write(css_file_path, STYLE_CSS)?;

    Ok(())
}

pub fn generate_readme_file(project_path: &Path, project_name: &str) -> io::Result<()> {
    let content = README.replace("project_name", project_name);

    let readme_file_path = project_path.join("README.md");

    fs::write(readme_file_path, content)?;

    Ok(())
}

pub fn generate_license_file(project_path: &Path) -> io::Result<()> {
    let license_file_path = project_path.join("LICENSE");

    fs::write(license_file_path, LICENSE)?;

    Ok(())
}

pub fn generate_gitignore_file(project_path: &Path) -> io::Result<()> {
    let gitignore_file_path = project_path.join(".gitignore");

    fs::write(gitignore_file_path, GITIGNORE)?;

    Ok(())
}

pub fn generate_env_files(
    project_path: &Path,
    db_name: &str,
    db_user: &str,
    db_pass: &str,
    app_port: &str,
    db_port: &str,
) -> io::Result<()> {
    let env_content = ENV_FILE
        .replace("project_db_name", db_name)
        .replace("project_db_user", db_user)
        .replace("project_db_pass", db_pass)
        .replace("project_app_port", app_port)
        .replace("project_db_port", db_port);

    fs::write(project_path.join(".env"), env_content)?;

    fs::write(project_path.join(".env.example"), ENV_EXAMPLE_FILE)?;

    Ok(())
}

pub fn generate_docker_files(
    project_path: &Path,
    project_name: &str,
    db_name: &str,
    db_user: &str,
    db_pass: &str,
    app_port: &str,
    db_port: &str,
) -> io::Result<()> {
    fs::write(project_path.join(".dockerignore"), DOCKERIGNORE)?;
    fs::write(project_path.join("Dockerfile"), DOCKERFILE)?;

    let compose_content = DOCKER_COMPOSE
        .replace("project_name", project_name)
        .replace("project_db_name", db_name)
        .replace("project_db_user", db_user)
        .replace("project_db_pass", db_pass)
        .replace("project_app_port", app_port)
        .replace("project_db_port", db_port);

    fs::write(project_path.join("docker-compose.yml"), compose_content)?;

    Ok(())
}

pub fn download_mysql_connector(project_path: &Path) -> io::Result<()> {
    let lib_dir = project_path.join("web").join("WEB-INF").join("lib");

    fs::create_dir_all(&lib_dir)?;

    let jar_path = lib_dir.join(MYSQL_JAR_NAME);

    println!("Downloading {MYSQL_JAR_NAME}...");

    let response = ureq::get(MYSQL_CONNECTOR_URL)
        .call()
        .map_err(|e| io::Error::other(format!("HTTP request failed: {e}")))?;

    let mut file = File::create(&jar_path)?;

    let mut reader = response.into_body().into_reader();
    io::copy(&mut reader, &mut file)?;

    Ok(())
}
