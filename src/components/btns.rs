use super::image::Image;
use my_clock::{is_in_chrome, set_screen};
use sycamore::prelude::*;

#[component]
pub fn Btns<G: Html>(ctx: Scope) -> View<G> {
    let class = match is_in_chrome() {
        true => "",
        false => "hidden",
    };
    let image = create_signal(ctx, Image::new());
    let is_dark = create_signal(ctx, false);
    let is_full = create_signal(ctx, false);
    create_effect(ctx, || {
        image.get().check(*is_dark.get(), *is_full.get());
    });
    view! {
        ctx,
        img(
            class=format!("screen-btn {}", class),
            src=image.get().screen.get(),
            on:click=|_|{
                set_screen(!*is_full.get());
                is_full.set(!*is_full.get());
            }
        )
        img(class="theme-btn",
            src=image.get().theme.get(),
            on:click=|_|{
                is_dark.set(!*is_dark.get());
            }
        )
    }
}
