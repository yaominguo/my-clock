mod btns;
mod clock;
mod flipper;
use btns::Btns;
use clock::Clock;
use sycamore::prelude::*;

#[component]
pub fn App<G: Html>(ctx: Scope) -> View<G> {
    view! {
        ctx,
        Btns()
        Clock()
    }
}
