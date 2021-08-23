pub mod flipper;
use chrono::{DateTime, Local};
use wasm_bindgen::{prelude::*, JsCast};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("Error: 无法访问到window对象！")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Error: window中未发现document对象")
}

pub fn request_animation_frame(cb: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .expect("Error: window中未发现requestAnimationFrame方法！");
}

pub fn get_time(fmt: &str) -> String {
    let time: DateTime<Local> = Local::now();
    time.format(fmt).to_string()
}
