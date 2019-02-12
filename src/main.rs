extern crate hex;
extern crate ring;

use ring::{digest, test};

fn main() -> std::io::Result<()> {
    let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
    let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
    // TODO swap SHA256 out with SHA1 possibly for speed
    // although SHA2 is probably fine (100ms difference in some benchmarks)
    let actual = digest::digest(&digest::SHA256, b"hello, world");
    let actual_hex = hex::encode(actual.as_ref());
    assert_eq!(&expected, &actual.as_ref());

    println!("{:x?}", actual_hex);

    Ok(())
}
