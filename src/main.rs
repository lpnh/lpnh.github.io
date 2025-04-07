#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::wildcard_imports)]

mod app;

use app::*;
use leptos::{logging, mount};

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("coucou");

    mount::mount_to_body(App);
}
