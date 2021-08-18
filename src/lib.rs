use chrono::{DateTime, Local};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn get_time(fmt: &str) -> String {
    let time: DateTime<Local> = Local::now();
    time.format(fmt).to_string()
}

#[wasm_bindgen]
pub fn set_clock(el: &str) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let clock = document()
        .query_selector(el)
        .unwrap()
        .expect("Error: document中未发现#clock对象！");

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let time = get_time("%H:%M");
        clock.set_text_content(Some(&time));
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("Error: 无法访问到window对象！")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Error: window中未发现document对象")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Error: window中未发现requestAnimationFrame方法！");
}
