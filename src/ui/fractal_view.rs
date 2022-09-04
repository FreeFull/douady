use imgref::Img;
use rug::Complex;
use std::cell::RefCell;
use std::ops::DivAssign;
use vizia::prelude::*;
use vizia::vg;
use vizia::vg::rgb::RGB;
use vizia::vg::ImageFlags;

use crate::ITERATIONS;
use crate::PRECISION;

pub struct FractalView {
    inner: RefCell<Inner>,
}

struct Inner {
    image_id: Option<vg::ImageId>,
    image: Img<Vec<vg::rgb::RGB8>>,
}

impl Inner {
    fn new() -> Self {
        Inner {
            image_id: None,
            image: Img::new(Vec::new(), 1, 0),
        }
    }
}

impl FractalView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        FractalView {
            inner: RefCell::new(Inner::new()),
        }
        .build(cx, |_| {})
    }
}

impl View for FractalView {
    fn element(&self) -> Option<&'static str> {
        Some("fractalview")
    }

    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {}

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let mut inner = self.inner.borrow_mut();

        let mut buf = std::mem::replace(inner.image.buf_mut(), Vec::new());
        buf.resize(bounds.w as usize * bounds.h as usize, RGB::new(0, 0, 0));
        inner.image = Img::new(buf, bounds.w as usize, bounds.h as usize);
        let divscale = inner.image.width().min(inner.image.height());

        for (y, row) in inner.image.rows_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let mut c = Complex::with_val(PRECISION, (x, y));
                c /= divscale as f64 / 4.0;
                c -= Complex::with_val(PRECISION, (2, 2));
                let mut i = crate::fractals::mandelbrot::draw(c) / ITERATIONS as f64 * 255.0;
                i = i.clamp(0.0, 255.0);
                *pixel = RGB::new(i as u8, i as u8, i as u8);
            }
        }
        if let Some(image_id) = inner.image_id {
            let info = canvas.get_image(image_id).unwrap().info();
            if info.width() == inner.image.width() && info.height() == inner.image.height() {
                canvas
                    .update_image(image_id, inner.image.as_ref(), 0, 0)
                    .unwrap();
            } else {
                canvas.delete_image(image_id);
                let image_id = canvas
                    .create_image(inner.image.as_ref(), vg::ImageFlags::empty())
                    .unwrap();
                inner.image_id = Some(image_id);
            }
        } else {
            inner.image_id = Some(
                canvas
                    .create_image(inner.image.as_ref(), vg::ImageFlags::empty())
                    .unwrap(),
            );
        }
        let mut path = vg::Path::new();
        path.rect(0.0, 0.0, bounds.w, bounds.h);
        canvas.fill_path(
            &mut path,
            vg::Paint::image(
                inner.image_id.unwrap(),
                0.0,
                0.0,
                bounds.w,
                bounds.h,
                0.0,
                1.0,
            ),
        );
    }
}
