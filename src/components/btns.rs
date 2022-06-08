use my_clock::{document, navigator};
use sycamore::prelude::*;
use tracing::info;

#[component]
pub fn Btns<G: Html>(ctx: Scope) -> View<G> {
    let navigator = navigator();
    let show_screen_btn = match navigator.user_agent().unwrap().contains("Chrome") {
        true => "",
        false => "hidden",
    };
    info!("navigator: {:?}", navigator.user_agent().unwrap());
    let image = Image::new();
    let is_dark = create_signal(ctx, false);
    let is_full = create_signal(ctx, false);
    let src = create_memo(ctx, move || {
        image.check(*is_dark.get(), *is_full.get());
        image.set(&image.theme.get(), &image.screen.get())
    });
    view! {
        ctx,
        img(class=format!("screen-btn {}", show_screen_btn), src=src.get().screen.get(), on:click=|_|{
            set_screen(!*is_full.get());
            is_full.set(!*is_full.get());
        })
        img(class="theme-btn", src=src.get().theme.get(), on:click=|_|{
            is_dark.set(!*is_dark.get());
        })
    }
}

fn set_screen(is_full: bool) {
    info!(
        "{:?}",
        document().document_element().unwrap().request_fullscreen()
    );
    if is_full {
        document()
            .document_element()
            .unwrap()
            .request_fullscreen()
            .unwrap();
    } else {
        document().exit_fullscreen();
    }
}

struct Image {
    theme: RcSignal<String>,
    screen: RcSignal<String>,
}

impl Image {
    fn new() -> Self {
        Self {
            theme: create_rc_signal(Self::src("moon")),
            screen: create_rc_signal(Self::src("full")),
        }
    }
    fn set(&self, theme: &str, screen: &str) -> Self {
        Self {
            theme: create_rc_signal(theme.to_string()),
            screen: create_rc_signal(screen.to_string()),
        }
    }
    fn check(&self, is_dark: bool, is_full: bool) {
        if is_dark {
            document().body().unwrap().set_class_name("dark");
            self.theme.set(Self::src("sun"));
            if is_full {
                self.screen.set(Self::src("exit-dark"));
            } else {
                self.screen.set(Self::src("full-dark"));
            }
        } else {
            document().body().unwrap().set_class_name("");
            self.theme.set(Self::src("moon"));
            if is_full {
                self.screen.set(Self::src("exit"));
            } else {
                self.screen.set(Self::src("full"));
            }
        }
    }
    fn src(name: &str) -> String {
        format!("assets/{}.svg", name)
    }
}
