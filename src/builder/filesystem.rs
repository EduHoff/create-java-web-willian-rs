use std::{fs, io, path::Path};

pub fn create_project_structure(project_path: &Path) -> io::Result<()> {
    let java_src_dir = project_path.join("src/java");
    let conf_dir = project_path.join("src/conf");

    let web_css_dir = project_path.join("web/assets/css");
    let web_js_dir = project_path.join("web/assets/js");
    let web_img_dir = project_path.join("web/assets/img");

    let meta_inf_dir = project_path.join("web/META-INF");
    let web_inf_lib_dir = project_path.join("web/WEB-INF/lib");

    fs::create_dir_all(&java_src_dir)?;
    fs::create_dir_all(&conf_dir)?;

    fs::create_dir_all(&web_css_dir)?;
    fs::create_dir_all(&web_js_dir)?;
    fs::create_dir_all(&web_img_dir)?;

    fs::create_dir_all(&meta_inf_dir)?;
    fs::create_dir_all(&web_inf_lib_dir)?;

    Ok(())
}
