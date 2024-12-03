use miette::{miette, IntoDiagnostic, Result};
use std::{collections::HashSet, io::Read, path::PathBuf};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn default_file(file: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(filename) = file {
        if !filename.exists() {
            return Err(miette!(
                "File '{}' doesn't exist",
                filename.to_string_lossy()
            ));
        }
    }

    let mut tried_files = HashSet::new();

    {
        let cur_dir_file = std::env::current_dir().into_diagnostic()?.join("input.txt");
        if cur_dir_file.exists() {
            return Ok(cur_dir_file);
        }
        tried_files.insert(cur_dir_file);
    }

    {
        let mut app_dir_file = std::env::current_exe().into_diagnostic()?;
        app_dir_file.set_file_name("input.txt");
        if app_dir_file.exists() {
            return Ok(app_dir_file);
        }
        tried_files.insert(app_dir_file);
    }

    {
        let project_bin_file = PathBuf::from(CARGO_MANIFEST_DIR).join("input.txt");
        if project_bin_file.exists() {
            return Ok(project_bin_file);
        }
        tried_files.insert(project_bin_file);
    }

    {
        let mut project_workspace_file = PathBuf::from(CARGO_MANIFEST_DIR);
        project_workspace_file.set_file_name("input.txt");
        if project_workspace_file.exists() {
            return Ok(project_workspace_file);
        }
        tried_files.insert(project_workspace_file);
    }

    let list = tried_files
        .into_iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("\n\t");
    Err(miette!("Couldn't find default file. Tried:\n\t{list}",))
}

pub fn read_input(file: Option<PathBuf>) -> Result<String> {
    let file = default_file(file)?;

    let mut file = std::fs::File::open(file).into_diagnostic()?;
    let mut content = String::new();
    file.read_to_string(&mut content).into_diagnostic()?;

    Ok(content)
}
