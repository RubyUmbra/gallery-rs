use clap::{Arg, Command, crate_authors, crate_description, crate_name, crate_version};
use std::env;
use std::path::PathBuf;

fn main() {
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("path")
                .help("path of directory with pictures to sort")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        );

    let matches = app.get_matches();

    let path = matches
        .get_one::<PathBuf>("path")
        .cloned()
        .ok_or("")
        .or(env::current_dir())
        .expect("Error accessing current directory");

    match gallery_rs::run(&path) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
