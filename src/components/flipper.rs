use my_clock::set_timeout;
use std::ops::Deref;
use sycamore::prelude::RcSignal;
use wasm_bindgen::prelude::*;

#[derive(Default, PartialEq, Clone)]
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

    // 判断时间数字与当前不一样则flip
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
        set_timeout(cb, 600);
    }
}
