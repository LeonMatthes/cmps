#[macro_use]
extern crate clap;
use clap::App;
use cmps;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filename = matches.value_of("FILENAME").unwrap();
    cmps::compose(
        filename,
        matches.value_of("EXTENSION").or_else(|| {
            Path::new(filename)
                .extension()
                .and_then(|extension| extension.to_str())
        }),
    )
    .unwrap();
}
