use std::{io::Read, path::PathBuf};

use anyhow::{Result, anyhow};

const CRATE_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn default_file(file: Option<PathBuf>, caller_manifest_dir: &str) -> Result<PathBuf> {
    if let Some(filename) = file {
        if !filename.exists() {
            return Err(anyhow!(
                "File '{}' doesn't exist",
                filename.to_string_lossy()
            ));
        }
        return Ok(filename);
    }

    let mut tried_files = Vec::new();

    // File in the current directory
    {
        let cur_dir_file = std::env::current_dir()?.join("input.txt");
        if cur_dir_file.exists() {
            return Ok(cur_dir_file);
        }
        tried_files.push(cur_dir_file);
    }

    // File in the same directory as the executable
    {
        let mut app_dir_file = std::env::current_exe()?;
        app_dir_file.set_file_name("input.txt");
        if app_dir_file.exists() {
            return Ok(app_dir_file);
        }
        tried_files.push(app_dir_file);
    }

    // File in the root directory of the caller crate
    {
        let project_bin_file = PathBuf::from(caller_manifest_dir).join("input.txt");
        if project_bin_file.exists() {
            return Ok(project_bin_file);
        }
        tried_files.push(project_bin_file);
    }

    // File in the AoC input repository
    {
        let path = PathBuf::from(caller_manifest_dir);
        let day = path.file_name().expect("unexpected build environment");
        let input_repo = path
            .parent()
            .ok_or_else(|| anyhow!(format!("Invalid build path '{caller_manifest_dir}'")))?
            .parent()
            .ok_or_else(|| anyhow!(format!("Invalid build path '{caller_manifest_dir}'")))?
            .join("input")
            .join(env!("AOC_YEAR"))
            .join(day)
            .join("input.txt");
        if input_repo.exists() {
            return Ok(input_repo);
        }
        tried_files.push(input_repo);
    }

    // File in the root directory of the project/workspace
    {
        let mut project_workspace_file = PathBuf::from(CRATE_MANIFEST_DIR);
        project_workspace_file.set_file_name("input.txt");
        if project_workspace_file.exists() {
            return Ok(project_workspace_file);
        }
        tried_files.push(project_workspace_file);
    }

    let list = tried_files
        .into_iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("\n\t");
    Err(anyhow!(
        "Couldn't find default input file. Tried:\n\t{list}",
    ))
}

// Use a macro to read the manifest dir of the caller crate, not this crate
#[macro_export]
macro_rules! read_input_str {
    ($arg_file:expr) => {
        ::common::input::read_input_str_fn($arg_file, env!("CARGO_MANIFEST_DIR"))
    };
}

pub fn read_input_str_fn(
    file: Option<PathBuf>,
    build_env: &str,
) -> Result<(Option<String>, String)> {
    let filename = default_file(file, build_env)?;

    let mut file = std::fs::File::open(filename.clone())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok((Some(filename.to_string_lossy().to_string()), content))
}

// Use a macro to read the manifest dir of the caller crate, not this crate
#[macro_export]
macro_rules! read_input_u8 {
    ($arg_file:expr) => {
        ::common::input::read_input_u8_fn($arg_file, env!("CARGO_MANIFEST_DIR"))
    };
}

pub fn read_input_u8_fn(
    file: Option<PathBuf>,
    build_env: &str,
) -> Result<(Option<String>, Vec<u8>)> {
    let filename = default_file(file, build_env)?;

    let mut file = std::fs::File::open(filename.clone())?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    Ok((Some(filename.to_string_lossy().to_string()), content))
}
