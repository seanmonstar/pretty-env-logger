extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;

mod one {
    pub fn deep() {
        trace!("one level deep!");
        trace!("one level deep!");
    }
}

fn main() {
    env::set_var("RUST_APP_LOG", "trace");

    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    info!("such information");
    info!("such information");
    warn!("o_O");
    warn!("o_O");
    error!("boom");
    error!("boom");
    debug!("deboogging");
    debug!("deboogging");
    self::one::deep();

    env::remove_var("RUST_APP_LOG");
}
