use chrono::{DateTime, Local};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_time(fmt: &str) -> String {
    let time: DateTime<Local> = Local::now();
    time.format(fmt).to_string()
}

#[wasm_bindgen]
pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}
