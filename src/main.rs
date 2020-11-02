#[macro_use]
extern crate clap;
use clap::App;
use compose;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filename = matches.value_of("FILENAME").unwrap();
    compose::compose(
        filename,
        matches.value_of("EXTENSION").or_else(|| {
            Path::new(filename)
                .extension()
                .and_then(|extension| extension.to_str())
        }),
    )
    .unwrap();
}
