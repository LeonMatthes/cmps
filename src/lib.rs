#![warn(missing_docs)]
//! # cmps
//!
//! This library is the backend for the cmps cli tool.
//! It can be used to create files and automatically fill them with a default template.
//! The template is determined from the file extension.
use dirs;
use log::*;
use std::env;
use std::{
    fs,
    fs::File,
    io,
    path::{Path, PathBuf},
};

/// Get the template for a given extension.
pub fn template_contents(extension: &str) -> Option<Vec<u8>> {
    trace!("Entered template_contents function.");

    for path in template_paths(extension) {
        if path.exists() {
            if let Some(contents) = read_template_from(&path) {
                info!("Using template file '{}'", path.display());
                return Some(contents);
            }
        } else {
            debug!(
                "Template file '{}' does not exist, skipping...",
                path.display()
            );
        }
    }
    None
}

#[derive(PartialEq, Eq)]
/// This enum indicates the mode to use when creating a new file.
pub enum CreationMode {
    /// default - only overwrite existing files if they are empty.
    OverwriteEmptyOnly,
    /// If the Force option is specified, existing files will be overwriten.
    Force,
}

/// Try to create the file for the specified path.
pub fn create_file<P: AsRef<Path>>(path: P, mode: CreationMode) -> io::Result<File> {
    match fs::metadata(&path) {
        Ok(metadata) => {
            debug!("Existing file found");
            if metadata.len() == 0 || mode == CreationMode::Force {
                info!("Overwriting existing file");
                File::create(&path)
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "File already exists! - Use '-f'/'--force' to overwrite anyway.",
                ));
            }
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => File::create(&path),
            _ => return Err(error),
        },
    }
}

/// Display the template contents for this extension and the source file path.
///
/// Accessible by the --show option of the cmps command line interface.
pub fn show_extension_info(extension: &str) {
    let mut template: Option<(_, _)> = None;
    let mut shadowed_templates = Vec::new();

    let paths = template_paths(extension);
    for path in paths.iter().filter(|path| path.exists()) {
        if template.is_none() {
            template = read_template_from(path).map(|template| (path.clone(), template));
        } else {
            shadowed_templates.push(path);
        }
    }

    if let Some((path, template)) = template {
        println!(
            "Template file for extension '{}' found in '{}'",
            extension,
            path.display()
        );
        if let Ok(string) = std::str::from_utf8(template.as_slice()) {
            println!("~~~~~~~   template   ~~~~~~");
            // the file might be missing the terminating newline
            if string.ends_with("\n") {
                print!("{}", string);
            } else {
                println!("{}", string);
            }
            println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        } else {
            println!("Template is binary and cannot be printed.");
        }
        for shadowed in shadowed_templates {
            println!("Note: Template file '{}' shadowed", shadowed.display());
        }
    } else {
        println!("No template file found for extension '{}'!", extension);
        println!("\nTried the following paths:");
        for path in paths {
            println!("\t{}", path.display());
        }
    }
}

fn local_config_directories() -> Vec<PathBuf> {
    trace!("Entered local_config_directories function");

    if let Ok(mut current_dir) = std::env::current_dir() {
        let mut config_dirs = Vec::new();

        loop {
            debug!(
                "Searching '{}' for '.cmps' directory",
                current_dir.to_string_lossy()
            );

            let local_config_dir = current_dir.join(".cmps");
            let has_cmps_dir = local_config_dir.try_exists().unwrap_or_else(|e| {
                warn!(
                    "Failed to query existence of '.cmps' directory in {}! - Error: {e}",
                    current_dir.to_string_lossy()
                );
                false
            });
            if has_cmps_dir {
                info!(
                    "Found local configuration directory at: '{}'",
                    local_config_dir.to_string_lossy()
                );
                config_dirs.push(local_config_dir);
            }

            if !current_dir.pop() {
                return config_dirs;
            }
        }
    }
    Vec::new()
}

fn base_paths() -> Vec<PathBuf> {
    trace!("Entered base_paths function.");

    // The ordering here is important, templates in the first folder in this list have the highest priority
    let base_paths = [
        dirs::config_dir().map(|dir| dir.join("cmps")),
        dirs::data_local_dir().map(|dir| dir.join("cmps")),
        Some(PathBuf::from(env!("CARGO_MANIFEST_DIR"))),
    ]
    .into_iter()
    .flatten();

    local_config_directories()
        .into_iter()
        .chain(base_paths)
        .map(|mut path| {
            path.push("templates");
            path
        })
        .collect()
}

fn template_paths(extension: &str) -> Vec<PathBuf> {
    base_paths()
        .into_iter()
        .map(|path| path.join(extension))
        .collect()
}

fn read_template_from<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    let contents = fs::read(&path);
    match contents {
        Ok(contents) => Some(contents),
        Err(error) => {
            warn!(
                "Template file '{}' could not be read!\nError: {}",
                path.as_ref().display(),
                &error
            );
            None
        }
    }
}
