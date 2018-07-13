//! Example program drawing circles on a page.
extern crate pdf_canvas;

use pdf_canvas::graphicsstate::Color;
use pdf_canvas::Pdf;
use std::f32::consts::PI;

/// Create a `circles.pdf` file, with a single page containg a circle
/// stroked in black, overwritten with a circle in a finer yellow
/// stroke.
/// The black circle is drawn using the `Canvas.circle` method,
/// which approximates a circle with four bezier curves.
/// The yellow circle is drawn as a 200-sided polygon.
fn main() {
    // Open our pdf document.
    let mut document = Pdf::create("circles.pdf").unwrap();

    // Add a 400x400 pt page.

    // Render-page writes the pdf file structure for a page and
    // creates a Canvas which is sent to the function that is the last
    // argument of the render_page method.
    // That function then puts content on the page by calling methods
    // on the canvas.
    document
        .render_page(400.0, 400.0, |c| {
            let (x, y) = (200.0, 200.0);
            let r = 190.0;

            // Set a wide black pen and stroke a circle
            try!(c.set_stroke_color(Color::rgb(0, 0, 0)));
            try!(c.set_line_width(2.0));
            try!(c.circle(x, y, r));
            try!(c.stroke());

            // Set a finer yellow pen and stroke a 200-sided polygon
            try!(c.set_stroke_color(Color::rgb(255, 230, 150)));
            try!(c.set_line_width(1.0));
            try!(c.move_to(x + r, y));
            let sides = 200;
            for n in 1..sides {
                let phi = (2 * n) as f32 * PI / sides as f32;
                try!(c.line_to(x + r * phi.cos(), y + r * phi.sin()));
            }
            c.stroke()
        })
        .unwrap();
    document.finish().unwrap();
}
