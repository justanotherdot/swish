extern crate clap;
extern crate hex;
extern crate rayon;
extern crate ring;

use clap::{crate_name, App, Arg, ArgMatches};
use rayon::prelude::*;
use ring::digest;
use std::fs;

fn par_hash_lines(lines: Vec<&str>) -> Vec<String> {
    lines.par_iter().map(|s| to_hex_digest(s)).collect()
}

fn to_hex_digest(s: &str) -> String {
    let digest = digest::digest(&digest::SHA256, s.as_ref());
    hex::encode(digest.as_ref())
}

fn hash_file(buffer: &str) -> String {
    let lines = buffer.lines().collect();
    let mut shas = par_hash_lines(lines);
    shas.par_sort();
    let hex_digest = to_hex_digest(&shas.join(""));
    return hex_digest;
}

fn process_files(files: Vec<&str>) -> Vec<String> {
    files
        .par_iter()
        .map(|file| fs::read_to_string(file))
        .map(|buffer| hash_file(&buffer.unwrap()))
        .collect()
}

fn parse_opts<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
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
        .get_matches()
}

fn main() -> std::io::Result<()> {
    let matches = parse_opts();
    // TODO This should probably be our own Config struct, or similar.
    let files = matches.values_of("FILE").unwrap().collect::<Vec<_>>();

    let shas = process_files(files);
    println!("{:?}", shas);

    // TODO This work can also be short-circuited by checking file lengths.
    //assert_eq!(hex_digest1, hex_digest2);

    Ok(())
}
