mod util;
use chrono::{DateTime, Local};
use std::{cell::RefCell, rc::Rc};
use util::{document, flipper::Flipper, window};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn get_time(fmt: &str) -> String {
    let time: DateTime<Local> = Local::now();
    time.format(fmt).to_string()
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let mut flippers: Vec<Rc<Flipper>> = vec![];
    let flips = document()
        .query_selector_all(".flip .inner")
        .expect("Error: document中未发现 .flip 元素！");
    for i in 0..flips.length() {
        flippers.push(Rc::new(Flipper::new(
            flips.get(i).unwrap().parent_element().unwrap(),
        )));
    }

    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let time = get_time("%H%M");
        for (i, flipper) in flippers.iter().enumerate() {
            let front_text = *flipper.front_text.borrow();
            let back_text = time[i..i + 1].parse::<u8>().unwrap();
            if front_text != back_text {
                Flipper::flip(flipper, front_text, back_text);
            }
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Error: window中未发现requestAnimationFrame方法！");
}
