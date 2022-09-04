mod fractals;
mod ui;

use vizia::{prelude::*, state::StaticLens};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const PRECISION: u32 = 24;
const ITERATIONS: u32 = 16;

fn main() {
    Application::new(|cx| {
        ui::fractal_view::FractalView::new(cx);
    })
    .min_inner_size(StaticLens::new(&Some((WIDTH, HEIGHT))))
    .run();
}
