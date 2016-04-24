///! Example program drawing circles on a page.
extern crate pdf;

use pdf::Pdf;
use std::f32::consts::PI;

/// Create a `circles.pdf` file, with a single page containg a circle
/// stroked in black, overwritten with a circle in a finer yellow
/// stroke.
/// The black drawing is drawn using the `Canvas.circle` method,
/// which approximates a circle with four bezier curves.
/// The yellow drawing is drawn as a 200-sided polygon.
fn main() {
    let mut document = Pdf::create("circles.pdf").unwrap();
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
