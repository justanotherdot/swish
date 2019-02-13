extern crate hex;
extern crate ring;

use ring::digest;
use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // TODO Consider SHA256 instead
    // This is just using the same as what git would do.
    let actual = digest::digest(&digest::SHA1, buffer.as_ref());
    let hex_digest = hex::encode(actual.as_ref());

    println!("{:x?}", hex_digest);

    Ok(())
}
