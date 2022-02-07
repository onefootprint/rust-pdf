use crate::encoding::{Encoding, WIN_ANSI_ENCODING};
use crate::fontref::FontRef;
use crate::graphicsstate::Color;
use std::io::{self, Write};

/// A text object is where text is put on the canvas.
///
/// A TextObject should never be created directly by the user.
/// Instead, the [Canvas.text](struct.Canvas.html#method.text) method
/// should be called.
/// It will create a TextObject and call a callback, before terminating
/// the text object properly.
///
/// # Example
///
/// ```
/// # use pdf_canvas::{Pdf, BuiltinFont, FontSource};
/// # use pdf_canvas::graphicsstate::Matrix;
/// # let mut document = Pdf::create("foo.pdf").unwrap();
/// # document.render_page(180.0, 240.0, |canvas| {
/// let serif = canvas.get_font(BuiltinFont::Times_Roman);
/// // t will be a TextObject
/// canvas.text(|t| {
///     t.set_font(&serif, 14.0)?;
///     t.set_leading(18.0)?;
///     t.pos(10.0, 300.0)?;
///     t.show("Some lines of text in what might look like a")?;
///     t.show_line("paragraph of three lines. Lorem ipsum dolor")?;
///     t.show_line("sit amet. Blahonga.")?;
///     Ok(())
/// })?;
/// # Ok(())
/// # }).unwrap();
/// # document.finish().unwrap();
/// ```
pub struct TextObject<'a> {
    output: &'a mut dyn Write,
    encoding: Encoding,
}

impl<'a> TextObject<'a> {
    // Should not be called by user code.
    pub(crate) fn new(output: &'a mut dyn Write) -> Self {
        TextObject {
            output,
            encoding: WIN_ANSI_ENCODING.clone(),
        }
    }

    /// Set the font and font-size to be used by the following text
    /// operations.
    pub fn set_font(&mut self, font: &FontRef, size: f32) -> io::Result<()> {
        self.encoding = font.get_encoding().clone();
        writeln!(self.output, "{} {} Tf", font, size)
    }
    /// Set leading, the vertical distance from a line of text to the next.
    /// This is important for the [show_line](#method.show_line) method.
    pub fn set_leading(&mut self, leading: f32) -> io::Result<()> {
        writeln!(self.output, "{} TL", leading)
    }
    /// Set the rise above the baseline for coming text.  Calling
    /// set_rise again with a zero argument will get back to the old
    /// baseline.
    pub fn set_rise(&mut self, rise: f32) -> io::Result<()> {
        writeln!(self.output, "{} Ts", rise)
    }
    /// Set the amount of extra space between characters, in 1/1000
    /// text unit.
    pub fn set_char_spacing(&mut self, a_c: f32) -> io::Result<()> {
        writeln!(self.output, "{} Tc", a_c)
    }
    /// Set the amount of extra space between words, in 1/1000
    /// text unit.
    pub fn set_word_spacing(&mut self, a_w: f32) -> io::Result<()> {
        writeln!(self.output, "{} Tw", a_w)
    }

    /// Set color for stroking operations.
    pub fn set_stroke_color(&mut self, color: Color) -> io::Result<()> {
        let norm = |c| f32::from(c) / 255.0;
        match color {
            Color::RGB { red, green, blue } => writeln!(
                self.output,
                "{} {} {} SC",
                norm(red),
                norm(green),
                norm(blue),
            ),
            Color::Gray { gray } => writeln!(self.output, "{} G", norm(gray)),
        }
    }
    /// Set color for non-stroking operations.
    pub fn set_fill_color(&mut self, color: Color) -> io::Result<()> {
        let norm = |c| f32::from(c) / 255.0;
        match color {
            Color::RGB { red, green, blue } => writeln!(
                self.output,
                "{} {} {} sc",
                norm(red),
                norm(green),
                norm(blue),
            ),
            Color::Gray { gray } => writeln!(self.output, "{} g", norm(gray)),
        }
    }

    /// Move text position.
    ///
    /// The first time `pos` is called in a
    /// TextObject, (x, y) refers to the same point as for
    /// [Canvas::move_to](struct.Canvas.html#method.move_to), after that,
    /// the point is relative to the earlier pos.
    pub fn pos(&mut self, x: f32, y: f32) -> io::Result<()> {
        writeln!(self.output, "{} {} Td", x, y)
    }
    /// Show a text.
    pub fn show(&mut self, text: &str) -> io::Result<()> {
        write!(self.output, "(")?;
        self.output.write_all(&self.encoding.encode_string(text))?;
        writeln!(self.output, ") Tj")
    }

    /// Show one or more text strings, allowing individual glyph positioning.
    ///
    /// Each item in param should contain a string to show and a number
    /// to adjust the position.
    /// The adjustment is measured in thousands of unit of text space.
    /// Positive adjustment brings letters closer, negative widens the gap.
    ///
    /// # Example
    ///
    /// ```
    /// # use pdf_canvas::{Pdf, BuiltinFont, FontSource};
    /// # use pdf_canvas::graphicsstate::Matrix;
    /// # let mut document = Pdf::create("foo.pdf").unwrap();
    /// # document.render_page(180.0, 240.0, |canvas| {
    /// # let serif = canvas.get_font(BuiltinFont::Times_Roman);
    /// # canvas.text(|t| {
    /// #    t.set_font(&serif, 14.0)?;
    /// t.show_adjusted(&[("W", 130), ("AN", -40), ("D", 0)])
    /// # })
    /// # }).unwrap();
    /// # document.finish().unwrap();
    /// ```
    pub fn show_adjusted(&mut self, param: &[(&str, i32)]) -> io::Result<()> {
        write!(self.output, "[")?;
        for &(text, offset) in param {
            write!(self.output, "(")?;
            self.output.write_all(&self.encoding.encode_string(text))?;
            write!(self.output, ") {} ", offset)?
        }
        writeln!(self.output, "] TJ")
    }
    /// Show a text as a line.  See also [set_leading](#method.set_leading).
    pub fn show_line(&mut self, text: &str) -> io::Result<()> {
        write!(self.output, "(")?;
        self.output.write_all(&self.encoding.encode_string(text))?;
        writeln!(self.output, ") '")
    }
    /// Push the graphics state on a stack.
    pub fn gsave(&mut self) -> io::Result<()> {
        // TODO Push current encoding in self?
        writeln!(self.output, "q")
    }
    /// Pop a graphics state from the [gsave](#method.gsave) stack and
    /// restore it.
    pub fn grestore(&mut self) -> io::Result<()> {
        // TODO Pop current encoding in self?
        writeln!(self.output, "Q")
    }
}
