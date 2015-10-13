use std::io::{Write, self};

use ::fontref::FontRef;
use ::encoding::WIN_ANSI_ENCODING;

/// A text object is where text is put on the canvas.
pub struct TextObject<'a> {
    output: &'a mut Write,
}

impl<'a> TextObject<'a> {
    pub fn new(output: &'a mut Write) -> TextObject {
        TextObject { output: output }
    }
    /// Set the font and font-size to be used by the following text
    /// operations.
    pub fn set_font(&mut self, font: &FontRef, size: f32) -> io::Result<()> {
        write!(self.output, "{} {} Tf\n", font, size)
    }
    /// Set leading, the vertical distance from a line of text to the next.
    /// This is important for the `show_line` method.
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
    /// Set rgb color for stroking operations
    pub fn set_stroke_color(&mut self, r: u8, g: u8, b: u8) -> io::Result<()> {
        let norm = |c| { c as f32 / 255.0 };
        write!(self.output, "{} {} {} SC\n", norm(r), norm(g), norm(b))
    }
    /// Set rgb color for non-stroking operations
    pub fn set_fill_color(&mut self, r: u8, g: u8, b: u8) -> io::Result<()> {
        let norm = |c| { c as f32 / 255.0 };
        write!(self.output, "{} {} {} sc\n", norm(r), norm(g), norm(b))
    }
    /// Set gray level for stroking operations
    pub fn set_stroke_gray(&mut self, gray: u8) -> io::Result<()> {
        write!(self.output, "{} G\n", gray as f32 / 255.0)
    }
    /// Set gray level for non-stroking operations
    pub fn set_fill_gray(&mut self, gray: u8) -> io::Result<()> {
        write!(self.output, "{} g\n", gray as f32 / 255.0)
    }
    /// Move text position.  The first time `pos` is called in a
    /// TextObject, (x, y) refers to the same point as for
    /// `Canvas::move_to`, after that, the point is relative to the
    /// earlier pos.
    pub fn pos(&mut self, x: f32, y: f32) -> io::Result<()> {
        write!(self.output, "{} {} Td\n", x, y)
    }
    /// Show a text.
    pub fn show(&mut self, text: &str) -> io::Result<()> {
        try!(self.output.write_all(b"("));
        try!(self.output.write_all(&WIN_ANSI_ENCODING.encode_string(text)));
        try!(self.output.write_all(b") Tj\n"));
        Ok(())
    }
    // TODO This method should have a better name, and take any combination
    // of strings as integers as arguments.
    pub fn show_j(&mut self, text: &str, offset: i32) -> io::Result<()> {
        try!(self.output.write_all(b"[("));
        try!(self.output.write_all(&WIN_ANSI_ENCODING.encode_string(text)));
        write!(self.output, ") {}] TJ\n", offset)
    }
    /// Show a text as a line.  See also `set_leading`.
    pub fn show_line(&mut self, text: &str) -> io::Result<()> {
        try!(self.output.write_all(b"("));
        try!(self.output.write_all(&WIN_ANSI_ENCODING.encode_string(text)));
        try!(self.output.write_all(b") '\n"));
        Ok(())
    }
    /// Push the graphics state on a stack.
    pub fn gsave(&mut self) -> io::Result<()> {
        write!(self.output, "q\n")
    }
    /// Pop a graphics state from the `gsave` stack and restore it.
    pub fn grestore(&mut self) -> io::Result<()> {
        write!(self.output, "Q\n")
    }
}
