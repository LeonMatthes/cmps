#[macro_use]
extern crate clap;
use clap::App;
use cmps;
use log::*;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbose = matches.occurrences_of("verbose") as usize;

    stderrlog::new()
        .module(module_path!())
        .verbosity(verbose + 1)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();

    if let Some(show_extension) = matches.value_of("show") {
        cmps::show_extension_info(show_extension);
    } else {
        let filename = matches.value_of("FILENAME").unwrap();
        let extension = matches.value_of("EXTENSION").or_else(|| {
            Path::new(filename)
                .extension()
                .and_then(|extension| extension.to_str())
        });
        if let Err(error) = cmps::compose(filename, extension) {
            error!("{}", error);
            std::process::exit(1);
        }
    }
}
