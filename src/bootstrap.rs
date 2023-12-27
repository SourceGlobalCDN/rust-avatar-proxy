use std::env;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "rust-avatar-proxy=info");
        log::debug!("Setting default RUST_LOG to 'rust-avatar-proxy=info'")
    }

    pretty_env_logger::init();

    log::info!("Application initialized");
    Ok(())
}
