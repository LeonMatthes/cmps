#![warn(missing_docs)]
//! # cmps
//!
//! This library is the backend for the cmps cli tool.
//! It can be used to create files and automatically fill them with a default template.
//! The template is determined from the file extension.
use dirs;
use log::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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

fn fill_file(file: &mut File, extension: &str) -> io::Result<usize> {
    trace!("Entered fill_file function.");
    let contents = template_contents(extension).unwrap_or_else(|| {
        warn!(
            "No template file found for extension {}, creating an empty file",
            &extension
        );
        String::default()
    });

    return file.write(contents.as_bytes());
}

fn template_contents(extension: &str) -> Option<String> {
    trace!("Entered template_contents function.");
    let extension_path: PathBuf = ["templates", extension].iter().collect();
    let base_paths = [
        dirs::config_dir().unwrap().join("cmps"),
        dirs::data_local_dir().unwrap().join("cmps"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    ];
    let template_paths = base_paths.iter().map(|path| path.join(&extension_path));

    for path in template_paths {
        if path.exists() {
            let contents = fs::read_to_string(&path);
            match contents {
                Ok(contents) => {
                    info!("Using template file '{}'", path.display());
                    return Some(contents);
                }
                Err(error) => warn!(
                    "Template file '{}' could not be read!\nError: {}",
                    path.display(),
                    &error
                ),
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
