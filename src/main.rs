extern crate hex;
extern crate ring;

use ring::digest;

fn main() -> std::io::Result<()> {
    // TODO Consider SHA256
    let actual = digest::digest(&digest::SHA1, b"some line");
    let hex_digest = hex::encode(actual.as_ref());

    println!("{:x?}", hex_digest);

    Ok(())
}
