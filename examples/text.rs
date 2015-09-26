extern crate pdf;

use pdf::{Pdf, FontSource};
use std::fs::File;

fn main() {
    let mut file = File::create("text.pdf").unwrap();
    let mut document = Pdf::new(&mut file).unwrap();
    document.render_page(300.0, 400.0, |c| {
        try!(c.set_stroke_color(200, 200, 255));
        try!(c.rectangle(10.0, 10.0, 280.0, 380.0));
        try!(c.line(10.0, 300.0, 290.0, 300.0));
        try!(c.line(150.0, 10.0, 150.0, 390.0));
        try!(c.stroke());
        let helvetica = c.get_font(FontSource::Helvetica);
        try!(c.text(|t| {
            try!(t.set_font(helvetica, 12.0));
            try!(t.pos(10.0, 380.0));
            t.show("Top left")
        }));
        try!(c.text(|t| {
            try!(t.pos(10.0, 10.0));
            t.show("Bottom left")
        }));
        try!(c.text(|t| {
            // TODO Use metrics to be able to center, right-lead and justify text.
            try!(t.pos(290.0 - 48.0, 380.0));
            t.show("Top right")
        }));
        try!(c.text(|t| {
            try!(t.pos(290.0 - 65.0, 10.0));
            t.show("Bottom right")
        }));
        let times = c.get_font(FontSource::Times_Roman);
        try!(c.text(|t| {
            try!(t.set_font(times, 14.0));
            try!(t.set_leading(18.0));
            try!(t.pos(10.0, 300.0));
            try!(t.show("Some lines of text in what might look like a"));
            try!(t.show_line("paragraph of three lines. Lorem ipsum dolor"));
            t.show_line("sit amet. Blahonga.")
        }));
        Ok(())
    }).unwrap();
    document.finish().unwrap();
}
