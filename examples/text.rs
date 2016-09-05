//! Example program drawing some text on a page.
extern crate pdf;

use pdf::{Pdf, BuiltinFont};
use pdf::graphicsstate::Color;

/// Create a `text.pdf` file, with a single page containg some
/// text lines positioned in various ways on some helper lines.
fn main() {
    let mut document = Pdf::create("text.pdf").unwrap();
    document.set_title("Text example");
    document.render_page(300.0, 400.0, |c| {
        try!(c.set_stroke_color(Color::rgb(200, 200, 255)));
        try!(c.rectangle(10.0, 10.0, 280.0, 380.0));
        try!(c.line(10.0, 300.0, 290.0, 300.0));
        try!(c.line(150.0, 10.0, 150.0, 390.0));
        try!(c.stroke());
        let helvetica = BuiltinFont::Helvetica;
        try!(c.left_text(10.0, 380.0, helvetica, 12.0, "Top left"));
        try!(c.left_text(10.0,  10.0, helvetica, 12.0, "Bottom left"));
        try!(c.right_text(290.0, 380.0, helvetica, 12.0, "Top right"));
        try!(c.right_text(290.0,  10.0, helvetica, 12.0, "Bottom right"));
        try!(c.center_text(150.0, 330.0, BuiltinFont::Times_Bold, 18.0,
                           "Centered"));
        let times = c.get_font(BuiltinFont::Times_Roman);
        try!(c.text(|t| {
            try!(t.set_font(&times, 14.0));
            try!(t.set_leading(18.0));
            try!(t.pos(10.0, 300.0));
            try!(t.show("Some lines of text in what might look like a"));
            try!(t.show_line("paragraph of three lines. Lorem ipsum dolor"));
            try!(t.show_line("sit amet. Blahonga. "));
            try!(t.show_adjusted(&[("W", 130), ("AN", -40), ("D", 0)]));
            try!(t.pos(0., -30.));
            t.show_adjusted(&(-19..21).map(|i| ("o", 16*i)).collect::<Vec<_>>())
        }));

        // In Swedish, we use the letters å, ä, and ö
        // in words like sloe liqueur.  That is why rust-pdf
        // uses /WinAnsiEncoding for text.
        let times_italic = BuiltinFont::Times_Italic;
        try!(c.right_text(290.0, 200.0, times_italic, 14.0,
                          "På svenska använder vi bokstäverna å, ä & ö"));
        try!(c.right_text(290.0, 182.0, times_italic, 14.0,
                          "i ord som slånbärslikör. Därför använder"));
        try!(c.right_text(290.0, 164.0, times_italic, 14.0,
                          "rust-pdf /WinAnsiEncoding för text."));


        try!(c.center_text(150.0, 130.0, BuiltinFont::Symbol, 14.0,
                           "Hellas ΑΒΓΔαβγδ"));
        try!(c.center_text(150.0, 114.0, BuiltinFont::Symbol, 14.0,
                           "∀ μ < δ : ∃ σ ∈ Σ"));
        Ok(())
    }).unwrap();
    document.finish().unwrap();
}
