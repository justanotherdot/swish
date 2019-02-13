extern crate hex;
extern crate ring;

use ring::digest;

fn main() -> std::io::Result<()> {
    // TODO Consider SHA256 instead
    // This is just using the same as what git would do.
    let actual = digest::digest(&digest::SHA1, b"some line");
    let hex_digest = hex::encode(actual.as_ref());

    println!("{:x?}", hex_digest);

    Ok(())
}
