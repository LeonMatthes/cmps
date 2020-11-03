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
        .verbosity(verbose)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    let filename = matches.value_of("FILENAME").unwrap();
    let extension = matches.value_of("EXTENSION").or_else(|| {
        Path::new(filename)
            .extension()
            .and_then(|extension| extension.to_str())
    });
    if let Err(error) = cmps::compose(filename, extension) {
        error!("{}", error);
    };
}
