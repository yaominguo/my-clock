use std::ops::Deref;

use sycamore::prelude::RcSignal;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Flipper {
    pub front: RcSignal<u32>,
    pub back: RcSignal<u32>,
    pub flipping: RcSignal<bool>,
}

impl Flipper {
    fn set_front(&self, val: u32) {
        self.front.set(val)
    }
    fn set_back(&self, val: u32) {
        self.back.set(val)
    }
    fn set_flipping(&self, val: bool) {
        self.flipping.set(val)
    }
}

#[derive(Debug)]
pub struct Flippers(Vec<Flipper>);

impl Deref for Flippers {
    type Target = Vec<Flipper>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Flippers {
    pub fn new() -> Self {
        Self(vec![
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ])
    }
    pub fn check_time(&self, time: &str) {
        for (i, c) in time.chars().enumerate() {
            let front = self[i].front.get();
            let back = c.to_digit(10).unwrap();
            if *front != back {
                self.flip(i, *front, back);
            }
        }
    }
    pub fn flip(&self, index: usize, front: u32, back: u32) {
        let cur = self[index].clone();
        if *cur.flipping.get() {
            return;
        }
        cur.set_flipping(true);
        cur.set_front(front);
        cur.set_back(back);

        let cb = Closure::wrap(Box::new(move || {
            cur.set_flipping(false);
            cur.set_front(back);
        }) as Box<dyn Fn()>);
        window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 600)
            .unwrap();
        cb.forget();
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(cb: &Closure<dyn Fn()>) {
    window()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .expect("no `request_animation_frame` method in global window");
}
