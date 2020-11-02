use dirs;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn compose<P: AsRef<Path>>(path: P, extension: Option<&str>) -> io::Result<File> {
    let mut file = match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.len() == 0 {
                File::create(&path)?
            } else {
                panic!("File already exists")
            }
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => File::create(&path)?,
            _ => panic!(
                "Cannot read the metadata of existing file {}\nError: {}",
                path.as_ref().display(),
                &error
            ),
        },
    };

    if let Some(extension) = extension {
        fill_file(&mut file, extension);
    }
    Ok(file)
}

fn fill_file(file: &mut File, extension: &str) {
    let contents = template_contents(extension).unwrap_or_else(|| {
        println!(
            "No template file found for extension {}, creating an empty file",
            &extension
        );
        String::default()
    });

    file.write(contents.as_bytes())
        .expect("Unable to write file");
}

fn template_contents(extension: &str) -> Option<String> {
    let extension_path: PathBuf = ["templates", extension].iter().collect();
    let base_paths = [
        dirs::config_dir().unwrap().join("compose"),
        dirs::data_local_dir().unwrap().join("compose"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    ];
    let template_paths = base_paths.iter().map(|path| path.join(&extension_path));

    for path in template_paths {
        if path.exists() {
            let contents = fs::read_to_string(&path);
            match contents {
                Ok(contents) => return Some(contents),
                Err(error) => println!(
                    "Template file '{}' could not be read!\nError: {}",
                    path.display(),
                    &error
                ),
            }
        } else {
            println!(
                "Template file '{}' does not exist, skipping...",
                path.display()
            );
        }
    }
    None
}
