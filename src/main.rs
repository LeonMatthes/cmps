use clap::Parser;
use cmps::{self, CreationMode};
use log::*;
use std::{io::Write, path::Path, process::ExitCode};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "An intelligent touch!

cmps is short for compose, a tool used to create or fill files with a default content.",
    help_template(
        "{name} {version}
{author}
{about}

{usage}

{all-args}"
    )
)]
struct Cli {
    #[arg(required_unless_present("show"))]
    /// The filename to compose, may point to a non-existing file, or an empty existing file.
    filename: Option<String>,

    /// The extension to use, overrides the extension in the filename (if any).
    extension: Option<String>,

    #[arg(
        long,
        value_name = "extension",
        conflicts_with("filename"),
        conflicts_with("extension")
    )]
    /// Show the template for this extension and the path to the source file.
    show: Option<String>,

    #[arg(long)]
    /// Write the template for this file to stdout. Does not create or modify the file.
    ///
    /// Useful for integrating with editors like (Neo-)vim.
    stdout: bool,

    #[arg(long, short)]
    /// Overwrite existing files.
    /// This will clear the file contents if no template is found.
    force: bool,

    #[arg(long,short,action=clap::ArgAction::Count)]
    /// Sets the level of verbosity (provide multiple times for higher levels)
    verbose: u8,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let verbose = cli.verbose as usize;
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbose + 1)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();

    if let Some(show_extension) = cli.show {
        cmps::show_extension_info(&*show_extension);
    } else {
        let filename = cli.filename.unwrap_or_default();
        let extension = cli.extension.or_else(|| {
            Path::new(&*filename)
                .extension()
                .and_then(|extension| extension.to_owned().into_string().ok())
        });

        let template = extension
            .as_deref()
            .map(|ext| cmps::template_contents(&*ext))
            .flatten();

        if cli.stdout {
            if let Err(error) = std::io::stdout().write_all(&*(template.unwrap_or(vec![]))) {
                error!("Failed to write to stdout: {}", error);
                return ExitCode::FAILURE;
            }
        } else {
            let mode = if cli.force {
                CreationMode::Force
            } else {
                CreationMode::OverwriteEmptyOnly
            };

            let file = cmps::create_file(&*filename, mode);

            let mut file = match file {
                Ok(file) => file,
                Err(error) => {
                    error!("{}", error);
                    return ExitCode::FAILURE;
                }
            };

            if template.is_none() {
                info!(
                    "No template file found for extension '{}', creating an empty file",
                    extension.unwrap_or_default()
                );
            }

            if let Err(error) = file.write_all(&*template.unwrap_or(vec![])) {
                error!("Could not write file: {}", error);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
