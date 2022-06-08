mod components;

use components::App;
use sycamore::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    sycamore::render(|ctx| view!(ctx, App()));
}
