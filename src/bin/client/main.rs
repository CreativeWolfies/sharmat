// The ~client~ (the thing that displays pieces & stuff)

extern crate iced;

use iced::{Application, Settings};

pub mod gui;

fn main() {
    gui::Sharmat::run(Settings::default())
}
