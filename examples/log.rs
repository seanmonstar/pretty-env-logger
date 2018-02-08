extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod nested {
    pub fn deep() {
        trace!("one level deep!");
    }
}

fn main() {
    pretty_env_logger::init();

    if !log_enabled!(log::Level::Trace) {
        eprintln!("To see the full demo, try setting `RUST_LOG=log=trace`.");
        return;
    }

    self::nested::deep();
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
