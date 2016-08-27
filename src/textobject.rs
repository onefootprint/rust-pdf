use std::io::{self, Write};

use fontref::FontRef;
use encoding::{Encoding, WIN_ANSI_ENCODING};
use graphicsstate::Color;

/// A text object is where text is put on the canvas.
///
/// A TextObject should never be created directly by the user.
/// Instead, the [text](struct.Canvas.html#method.text) method on a Canvas object should be called.
/// It will create a TextObject and call a callback, before terminating
/// the text object properly.
///
/// # Example
///
/// ```
/// # use pdf::{Pdf, BuiltinFont, FontSource};
/// # use pdf::graphicsstate::Matrix;
/// # let mut document = Pdf::create("foo.pdf").unwrap();
/// # document.render_page(180.0, 240.0, |canvas| {
/// let serif = canvas.get_font(BuiltinFont::Times_Roman);
/// // t will be a TextObject
/// try!(canvas.text(|t| {
///     try!(t.set_font(&serif, 14.0));
///     try!(t.set_leading(18.0));
///     try!(t.pos(10.0, 300.0));
///     try!(t.show("Some lines of text in what might look like a"));
///     try!(t.show_line("paragraph of three lines. Lorem ipsum dolor"));
///     try!(t.show_line("sit amet. Blahonga."));
///     Ok(())
/// }));
/// # Ok(())
/// # }).unwrap();
/// # document.finish().unwrap();
/// ```
pub struct TextObject<'a> {
    output: &'a mut Write,
    encoding: Encoding,
}

impl<'a> TextObject<'a> {
    /// See [Canvas::text](struct.Canvas.html#method.text).
    /// User code is not supposed to call this constructor.
    pub fn new(output: &'a mut Write) -> TextObject {
        TextObject {
            output: output,
            encoding: WIN_ANSI_ENCODING.clone(),
        }
    }

    /// Set the font and font-size to be used by the following text
    /// operations.
    pub fn set_font(&mut self, font: &FontRef, size: f32) -> io::Result<()> {
        self.encoding = font.get_encoding();
        write!(self.output, "{} {} Tf\n", font, size)
    }
    /// Set leading, the vertical distance from a line of text to the next.
    /// This is important for the [show_line](#method.show_line) method.
    pub fn set_leading(&mut self, leading: f32) -> io::Result<()> {
        write!(self.output, "{} TL\n", leading)
    }
    /// Set the rise above the baseline for coming text.  Calling
    /// set_rise again with a zero argument will get back to the old
    /// baseline.
    pub fn set_rise(&mut self, rise: f32) -> io::Result<()> {
        write!(self.output, "{} Ts\n", rise)
    }
    /// Set the amount of extra space between characters, in 1/1000
    /// text unit.
    pub fn set_char_spacing(&mut self, a_c: f32) -> io::Result<()> {
        write!(self.output, "{} Tc\n", a_c)
    }
    /// Set the amount of extra space between words, in 1/1000
    /// text unit.
    pub fn set_word_spacing(&mut self, a_w: f32) -> io::Result<()> {
        write!(self.output, "{} Tw\n", a_w)
    }
    /// Set color for stroking operations.
    pub fn set_stroke_color(&mut self, color: Color) -> io::Result<()> {
        let norm = |c| c as f32 / 255.0;
        match color {
            Color::RGB { red, green, blue } => {
                write!(self.output,
                       "{} {} {} SC\n",
                       norm(red),
                       norm(green),
                       norm(blue))
            }
            Color::Gray { gray } => write!(self.output, "{} G\n", norm(gray)),
        }
    }
    /// Set color for non-stroking operations.
    pub fn set_fill_color(&mut self, color: Color) -> io::Result<()> {
        let norm = |c| c as f32 / 255.0;
        match color {
            Color::RGB { red, green, blue } => {
                write!(self.output,
                       "{} {} {} sc\n",
                       norm(red),
                       norm(green),
                       norm(blue))
            }
            Color::Gray { gray } => write!(self.output, "{} g\n", norm(gray)),
        }
    }
    /// Set gray level for stroking operations
    pub fn set_stroke_gray(&mut self, gray: u8) -> io::Result<()> {
        write!(self.output, "{} G\n", gray as f32 / 255.0)
    }
    /// Set gray level for non-stroking operations
    pub fn set_fill_gray(&mut self, gray: u8) -> io::Result<()> {
        write!(self.output, "{} g\n", gray as f32 / 255.0)
    }
    /// Move text position.
    ///
    /// The first time `pos` is called in a
    /// TextObject, (x, y) refers to the same point as for
    /// [Canvas::move_to](struct.Canvas.html#method.move_to), after that,
    /// the point is relative to the earlier pos.
    pub fn pos(&mut self, x: f32, y: f32) -> io::Result<()> {
        write!(self.output, "{} {} Td\n", x, y)
    }
    /// Show a text.
    pub fn show(&mut self, text: &str) -> io::Result<()> {
        try!(self.output.write_all(b"("));
        try!(self.output.write_all(&self.encoding.encode_string(text)));
        try!(self.output.write_all(b") Tj\n"));
        Ok(())
    }
    /// TODO This method should have a better name, and take any combination
    /// of strings as integers as arguments.
    pub fn show_j(&mut self, text: &str, offset: i32) -> io::Result<()> {
        try!(self.output.write_all(b"[("));
        try!(self.output.write_all(&self.encoding.encode_string(text)));
        write!(self.output, ") {}] TJ\n", offset)
    }
    /// Show a text as a line.  See also [set_leading](#method.set_leading).
    pub fn show_line(&mut self, text: &str) -> io::Result<()> {
        try!(self.output.write_all(b"("));
        try!(self.output.write_all(&self.encoding.encode_string(text)));
        try!(self.output.write_all(b") '\n"));
        Ok(())
    }
    /// Push the graphics state on a stack.
    pub fn gsave(&mut self) -> io::Result<()> {
        // TODO Push current encoding in self?
        write!(self.output, "q\n")
    }
    /// Pop a graphics state from the [gsave](#method.gsave) stack and
    /// restore it.
    pub fn grestore(&mut self) -> io::Result<()> {
        // TODO Pop current encoding in self?
        write!(self.output, "Q\n")
    }
}
