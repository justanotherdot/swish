mod config;
mod swish;

use config::Config;
use swish::{all_shas_match, hash_files};

fn main() -> std::io::Result<()> {
    let config = Config::new();

    let shas = hash_files(&config.files);

    if !all_shas_match(&shas) {
        std::process::exit(1);
    }

    Ok(())
}
