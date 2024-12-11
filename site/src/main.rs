mod app;
mod components;
mod pages;
mod sections;
mod theme;

use leptos::{logging::log, prelude::*};

use crate::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    mount_to_body(|| view! { <App/> })
}
