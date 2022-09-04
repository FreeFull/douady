use rug::{Complex, Float};

use crate::{ITERATIONS, PRECISION};

pub fn draw(c: Complex) -> f64 {
    let mut z = Complex::with_val(PRECISION, (0, 0));
    let mut n = 0;
    while n < ITERATIONS {
        if Float::with_val(2, z.norm_ref()).to_f32() > 16.0 {
            break;
        }
        z = z.square() + &c;
        n += 1;
    }
    n as f64 + 1.0 - z.abs().into_real_imag().0.to_f64().ln().log2()
}
