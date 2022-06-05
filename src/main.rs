mod flipper;
use std::{cell::RefCell, rc::Rc};

use crate::flipper::Flippers;
use chrono::Local;
use flipper::request_animation_frame;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    sycamore::render(|ctx| view!(ctx, App()));
}

#[component]
fn App<G: Html, 'a>(ctx: Scope<'a>) -> View<G> {
    let time = create_signal(ctx, get_time("%H%M"));
    let flippers = Flippers::new();

    let handler: Box<dyn Fn() + 'static> = unsafe {
        std::mem::transmute(Box::new(move || time.set(get_time("%H%M"))) as Box<dyn Fn()>)
    };
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        handler();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn Fn() + 'static>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    let list = create_memo(ctx, move || {
        flippers.check_time(&time.get());
        flippers
            .iter()
            .map(|flipper| flipper.clone())
            .collect::<Vec<_>>()
    });
    view! {
        ctx,
        div(class="clock") {
            Indexed {
                iterable: list,
                view: |ctx, flipper| view!{
                    ctx,
                    div(class={format!("flip {}", if *flipper.flipping.get() { "go" } else { "" })}) {
                        div(class={format!("digital front number{}", flipper.front.get())})
                        div(class={format!("digital back number{}", flipper.back.get())})
                    }
                },
            }
        }
    }
}

fn get_time(fmt: &str) -> String {
    let time = Local::now();
    time.format(fmt).to_string()
}
