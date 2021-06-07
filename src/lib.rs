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
    io::prelude::*,
    path::{Path, PathBuf},
};

mod template;

/// The main function of this library.
/// It takes a path to either a non-existing file, or an empty existing file.
/// The extension should usually match the extension in the path, but can be overriden by providing a different extension.
/// If extension is None, an empty file will be created.
pub fn compose<P: AsRef<Path>>(path: P, extension: Option<&str>) -> io::Result<File> {
    trace!("Entered compose function.");
    let mut file = match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.len() == 0 {
                File::create(&path)?
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "File already exists!",
                ));
            }
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => File::create(&path)?,
            _ => return Err(error),
        },
    };

    if let Some(extension) = extension {
        // Convert Result<usize> into Result<File>
        if let Err(error) = fill_file(&mut file, extension) {
            return Err(error);
        };
    }

    Ok(file)
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

fn base_paths() -> Vec<PathBuf> {
    trace!("Entered base_paths function.");

    // the ordering here is important, templates in the first folder in this list have the highest priority
    let base_paths = [
        dirs::config_dir().unwrap().join("cmps"),
        dirs::data_local_dir().unwrap().join("cmps"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    ];

    base_paths
        .iter()
        .map(|path| path.join("templates"))
        .collect()
}

fn template_paths(extension: &str) -> Vec<PathBuf> {
    base_paths()
        .into_iter()
        .map(|path| path.join(extension))
        .collect()
}

fn fill_file(file: &mut File, extension: &str) -> io::Result<usize> {
    trace!("Entered fill_file function.");
    let contents = template_contents(extension).unwrap_or_else(|| {
        info!(
            "No template file found for extension {}, creating an empty file",
            &extension
        );
        Vec::new()
    });

    return file.write(contents.as_slice());
}

fn template_contents(extension: &str) -> Option<Vec<u8>> {
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
