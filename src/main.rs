use std::cmp::Ordering;

use minifb::{Scale, Window, WindowOptions};
use rug::Complex;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const PRECISION: u32 = 16;
const ITERATIONS: u32 = 64;

fn iter(z: Complex, c: &Complex) -> Complex {
    z.square() + c
}

fn main() {
    let opts = WindowOptions {
        scale: Scale::X1,
        ..Default::default()
    };
    let mut window = Window::new("clone", WIDTH, HEIGHT, opts).expect("Unable to create window.");
    window.update();
    let mut buffer = [0; WIDTH * HEIGHT];
    let threshold = Complex::with_val(PRECISION, (2, 0));
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let xf = (x as f32) / (WIDTH as f32) * 3.0 - 2.5;
            let yf = (y as f32) / (HEIGHT as f32) * 3.0 - 1.5;
            let mut z = Complex::with_val(PRECISION, (0, 0));
            let c = Complex::with_val(PRECISION, (xf, yf));
            let mut i = 0.0;
            for n in 1..(ITERATIONS + 1) {
                z = iter(z, &c);
                if z.cmp_abs(&threshold) == Some(Ordering::Greater) {
                    let n = (n + 1) as f64;
                    i = n - z.abs().real().to_f64().ln().log(2.0);
                    break;
                }
            }
            buffer[x + y * WIDTH] = 0x00010101 * (i / ITERATIONS as f64 * 255.0) as u32;
        }
        window.update_with_buffer(&buffer).unwrap();
    }
    loop {
        window.update_with_buffer(&buffer).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
