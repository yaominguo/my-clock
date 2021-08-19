pub mod flipper;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("Error: 无法访问到window对象！")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Error: window中未发现document对象")
}
