use std::{cell::RefCell, rc::Rc};

use chrono::Local;
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn get_time(fmt: &str) -> String {
    let time = Local::now();
    time.format(fmt).to_string()
}

pub fn set_interval(handler: Box<dyn Fn()>) {
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        handler();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn Fn() + 'static>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

pub fn set_timeout(cb: Closure<dyn Fn()>, duration: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            duration,
        )
        .unwrap();
    cb.forget();
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("no `document` available in global window`")
}
pub fn navigator() -> web_sys::Navigator {
    window().navigator()
}

fn request_animation_frame(cb: &Closure<dyn Fn()>) {
    window()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .expect("no `request_animation_frame` method in global window");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
