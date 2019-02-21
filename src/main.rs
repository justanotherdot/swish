mod config;
mod swish;

use config::Config;
use swish::Swish;

fn main() -> std::io::Result<()> {
    let config = Config::new();

    if !Swish::new(config.files).hash().all_match() {
        std::process::exit(1);
    }

    Ok(())
}
