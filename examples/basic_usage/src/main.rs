use pixel_engine::{app::*, ui::*};

const PINK: u32 = 0xFFFF00FF;
const RED: u32 = 0xFFFF0000;
const BLUE: u32 = 0xFF00FF00;
const GREEN: u32 = 0xFF0000FF;

// TODO: update with custom params?
fn main() {
    App::new()
        .with_update(&update)
        .with_dimensions(256, 256)
        .with_scale(2)
        .run();
}

fn update(ui: &mut UI) {
    let font = FontOptions::new("../../fonts/PressStart2P-Regular.ttf").with_size(32.0);
    ui.write_string("hello", 0, 0);
    ui.set_font(&font);
    ui.write_string("hi", 0, 50);
}
