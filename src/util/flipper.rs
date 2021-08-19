use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Element;

use super::window;

pub struct Flipper {
    node: RefCell<web_sys::Element>,
    front_node: RefCell<web_sys::Element>,
    back_node: RefCell<web_sys::Element>,
    pub front_text: RefCell<u8>,
    pub back_text: RefCell<u8>,
    duration: i32,
    is_flipping: RefCell<bool>,
}

impl Flipper {
    pub fn new(el: web_sys::Element) -> Flipper {
        let front_node = el
            .query_selector(".front")
            .unwrap()
            .expect("Error: 未发现 .front 元素！");
        let back_node = el
            .query_selector(".back")
            .unwrap()
            .expect("Error: 未发现 .back 元素！");

        Flipper {
            node: RefCell::new(el),
            front_node: RefCell::new(front_node),
            back_node: RefCell::new(back_node),
            front_text: RefCell::new(0),
            back_text: RefCell::new(1),
            duration: 600,
            is_flipping: RefCell::new(false),
        }
    }

    pub fn set_front(&self, num: u8) {
        let class_name = format!("digital front number{}", num);
        *self.front_text.borrow_mut() = num;
        set_class(&self.front_node, class_name.as_str());
    }

    pub fn set_back(&self, num: u8) {
        let class_name = format!("digital back number{}", num);
        *self.back_text.borrow_mut() = num;
        set_class(&self.back_node, class_name.as_str());
    }

    pub fn flip(flipper: &Rc<Flipper>, front_num: u8, back_num: u8) {
        let item = Rc::clone(flipper);

        if !*item.is_flipping.borrow() {
            *item.is_flipping.borrow_mut() = true;
            item.set_front(front_num);
            item.set_back(back_num);

            item.node.borrow().set_class_name("flip down go");
            let duration = item.duration;

            let cb = Closure::wrap(Box::new(move || {
                item.node.borrow().set_class_name("flip down");
                *item.is_flipping.borrow_mut() = false;
                item.set_front(back_num);
                window().clear_timeout();
            }) as Box<dyn FnMut()>);

            window()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    duration,
                )
                .unwrap();

            cb.forget();
        }
    }
}

fn set_class(el: &RefCell<Element>, val: &str) {
    el.borrow().set_class_name(val);
}
