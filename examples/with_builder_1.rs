extern crate pretty_env_logger;
extern crate env_logger;
#[macro_use] extern crate log;

use env_logger::Target;

mod one {
    pub fn deep() {
        trace!("one level deep!");
        trace!("one level deep!");
    }
}

fn main() {

    pretty_env_logger::formatted_builder().unwrap()
        //let's just set some random stuff.. for more see
        //https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
        .target(Target::Stdout)
        .parse("with_builder_1=trace")
        .init();

    info!("such information");
    info!("such information");
    warn!("o_O");
    warn!("o_O");
    error!("boom");
    error!("boom");
    debug!("deboogging");
    debug!("deboogging");
    self::one::deep();
}
