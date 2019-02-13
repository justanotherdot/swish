extern crate clap;
extern crate hex;
extern crate ring;

use clap::{App, Arg};
use ring::digest;
use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let matches = App::new("swish")
        .version("0.1.0")
        .author("Ryan James Spencer <spencer.ryanjames@gmail.com>")
        .about("swish deterministically compares sets of files")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turns on verbose output"),
        )
        // TODO This will parse a single arg.
        // But it's probably better if we require it's greater than two.
        .arg(
            Arg::with_name("INPUT")
                .help("files to compare")
                .multiple(true)
                .required(true),
        )
        .get_matches();
    println!("{:?}", matches);

    // Read from stdin.
    // Eventually this will be file based
    // but supporting stdin appropriately e.g. `-f -`
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // TODO Consider SHA256 instead
    // This is just using the same as what git would do.
    let actual = digest::digest(&digest::SHA1, buffer.as_ref());
    let hex_digest = hex::encode(actual.as_ref());

    println!("{:x?}", hex_digest);

    Ok(())
}
