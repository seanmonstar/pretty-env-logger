extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod one {
    pub fn deep() {
        trace!("one level deep!");
    }
}

fn main() {
    pretty_env_logger::init();

    self::one::deep();
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
