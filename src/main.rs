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
    let buffer1 = fs::read_to_string(files[0])?;

    let hex_digest1 = hash_file(buffer1);
    println!("");
    println!("final hash 01: {}", hex_digest1);

    println!("");

    let buffer2 = fs::read_to_string(files[1])?;
    let hex_digest2 = hash_file(buffer2);
    println!("");
    println!("final hash 02: {}", hex_digest2);

    // TODO This work can also be short-circuited by checking file lengths.
    //assert_eq!(hex_digest1, hex_digest2);

    Ok(())
}
