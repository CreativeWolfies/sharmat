// The ~client~ (the thing that displays pieces & stuff)

extern crate iced;
extern crate iced_native;
extern crate iced_wgpu;

use iced::{Application, Settings};

pub mod gui;
pub mod style;
pub mod pieces;

fn main() {
    // TODO: do this in another thread or idk
    let piece_assets = pieces::load_assets(format!("{}/assets/", env!("CARGO_MANIFEST_DIR")));
    gui::Sharmat::run(Settings::with_flags(piece_assets))
}
