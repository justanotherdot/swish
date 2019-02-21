extern crate clap;

use clap::{crate_name, App, Arg};

pub struct Config {
    pub files: Vec<String>,
    pub verbose: bool,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new(crate_name!())
            .version("0.1.0")
            .author("Ryan James Spencer <spencer.ryanjames@gmail.com>")
            .about("deterministically compares sets of files")
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Turns on verbose output"),
            )
            .arg(
                Arg::with_name("FILE")
                    .min_values(2)
                    .required(true)
                    .empty_values(false),
            )
            .get_matches();

        let files = matches
            .values_of("FILE")
            .unwrap()
            .map(|s| s.to_owned())
            .collect();
        let verbose = matches.is_present("verbose");

        Config { files, verbose }
    }
}
