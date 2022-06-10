use my_clock::document;
use sycamore::prelude::*;

pub struct Image {
    pub theme: RcSignal<String>,
    pub screen: RcSignal<String>,
}

impl Image {
    pub fn new() -> Self {
        Self {
            theme: create_rc_signal(Self::src("moon", "")),
            screen: create_rc_signal(Self::src("full", "")),
        }
    }
    pub fn check(&self, is_dark: bool, is_full: bool) {
        let theme: &str;
        let screen: &str;
        let suffix: &str;
        if is_dark {
            document().body().unwrap().set_class_name("dark");
            theme = "sun";
            suffix = "-dark";
        } else {
            document().body().unwrap().set_class_name("");
            theme = "moon";
            suffix = ""
        }
        if is_full {
            screen = "exit";
        } else {
            screen = "full";
        }

        self.theme.set(Self::src(theme, ""));
        self.screen.set(Self::src(screen, suffix));
    }
    fn src(name: &str, suffix: &str) -> String {
        format!("assets/{}{}.svg", name, suffix)
    }
}
