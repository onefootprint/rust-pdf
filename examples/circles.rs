extern crate pdf;

use pdf::Pdf;
use std::fs::File;
use std::f32::consts::PI;

fn main() {
    let mut file = File::create("circles.pdf").unwrap();
    let mut document = Pdf::new(&mut file).unwrap();
    document.render_page(400.0, 400.0, |c| {
        let (x, y) = (200.0, 200.0);
        let r = 190.0;
        try!(c.set_stroke_color(0, 0, 0));
        try!(c.circle(x, y, r+0.5));
        try!(c.circle(x, y, r-0.5));
        try!(c.stroke());
        try!(c.set_stroke_color(255, 230, 150));
        try!(c.move_to(x + r, y));
        let sides = 200;
        for n in 1..sides {
            let phi = (2 * n) as f32 * PI / sides as f32;
            try!(c.line_to(x + r * phi.cos(), y + r * phi.sin()));
        }
        c.stroke()
    }).unwrap();
    document.finish().unwrap();
}
