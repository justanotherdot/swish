extern crate clap;
extern crate hex;
extern crate rayon;
extern crate ring;

use clap::{crate_name, App, Arg};
use rayon::prelude::*;
use ring::digest;
use std::fs;

fn hash_file(buffer: String) -> String {
    let lines = buffer.split("\n").collect::<Vec<_>>();
    let mut shas = lines
        .par_iter()
        .map(|s| hex::encode(digest::digest(&digest::SHA256, s.as_ref()).as_ref()))
        .collect::<Vec<_>>();
    shas.sort();
    for sha in shas.iter() {
        println!("hash: {}", sha);
    }

    let actual = digest::digest(&digest::SHA256, shas.join("").as_ref());
    let hex_digest = hex::encode(actual.as_ref());

    println!("");
    println!("final hash: {}", hex_digest);

    return hex_digest;
}

fn main() -> std::io::Result<()> {
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

    let files = matches.values_of("FILE").unwrap().collect::<Vec<_>>();
    for file in files.iter() {
        let buffer = fs::read_to_string(file)?;
        hash_file(buffer);
        println!("");
    }

    // TODO This work can also be short-circuited by checking file lengths.
    //assert_eq!(hex_digest1, hex_digest2);

    Ok(())
}
