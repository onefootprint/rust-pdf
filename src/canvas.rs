use fontref::FontRef;
use fontsource::{BuiltinFont, FontSource};
use graphicsstate::*;
use outline::OutlineItem;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use textobject::TextObject;

/// An visual area where content can be drawn (a page).
///
/// Provides methods for defining and stroking or filling paths, as
/// well as placing text objects.
///
/// TODO Everything here that takes a `BuiltinFont` should take any
/// `FontSource` instead.
pub struct Canvas<'a> {
    output: &'a mut Write,
    fonts: &'a mut HashMap<BuiltinFont, FontRef>,
    outline_items: &'a mut Vec<OutlineItem>,
}

// Should not be called by user code.
pub fn create_canvas<'a>(
    output: &'a mut Write,
    fonts: &'a mut HashMap<BuiltinFont, FontRef>,
    outline_items: &'a mut Vec<OutlineItem>,
) -> Canvas<'a> {
    Canvas {
        output,
        fonts,
        outline_items,
    }
}

impl<'a> Canvas<'a> {
    /// Append a closed rectangle with a corner at (x, y) and
    /// extending width × height to the to the current path.
    pub fn rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> io::Result<()> {
        writeln!(self.output, "{} {} {} {} re", x, y, width, height)
    }
    /// Set the line join style in the graphics state.
    pub fn set_line_join_style(
        &mut self,
        style: JoinStyle,
    ) -> io::Result<()> {
        writeln!(
            self.output,
            "{} j",
            match style {
                JoinStyle::Miter => 0,
                JoinStyle::Round => 1,
                JoinStyle::Bevel => 2,
            }
        )
    }
    /// Set the line join style in the graphics state.
    pub fn set_line_cap_style(&mut self, style: CapStyle) -> io::Result<()> {
        writeln!(
            self.output,
            "{} J",
            match style {
                CapStyle::Butt => 0,
                CapStyle::Round => 1,
                CapStyle::ProjectingSquare => 2,
            }
        )
    }
    /// Set the line width in the graphics state.
    pub fn set_line_width(&mut self, w: f32) -> io::Result<()> {
        writeln!(self.output, "{} w", w)
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

    /// Modify the current transformation matrix for coordinates by
    /// concatenating the specified matrix.
    pub fn concat(&mut self, m: Matrix) -> io::Result<()> {
        writeln!(self.output, "{} cm", m)
    }

    /// Append a straight line from (x1, y1) to (x2, y2) to the current path.
    pub fn line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> io::Result<()> {
        self.move_to(x1, y1)?;
        self.line_to(x2, y2)
    }
    /// Begin a new subpath at the point (x, y).
    pub fn move_to(&mut self, x: f32, y: f32) -> io::Result<()> {
        write!(self.output, "{} {} m ", x, y)
    }
    /// Add a straight line from the current point to (x, y) to the
    /// current path.
    pub fn line_to(&mut self, x: f32, y: f32) -> io::Result<()> {
        write!(self.output, "{} {} l ", x, y)
    }
    /// Add a Bézier curve from the current point to (x3, y3) with
    /// (x1, y1) and (x2, y2) as Bézier controll points.
    pub fn curve_to(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> io::Result<()> {
        writeln!(self.output, "{} {} {} {} {} {} c", x1, y1, x2, y2, x3, y3)
    }
    /// Add a circle approximated by four cubic Bézier curves to the
    /// current path.  Based on
    /// http://spencermortensen.com/articles/bezier-circle/
    pub fn circle(&mut self, x: f32, y: f32, r: f32) -> io::Result<()> {
        let top = y - r;
        let bottom = y + r;
        let left = x - r;
        let right = x + r;
        #[cfg_attr(feature = "cargo-clippy", allow(excessive_precision))]
        let c = 0.551_915_024_494;
        let dist = r * c;
        let up = y - dist;
        let down = y + dist;
        let leftp = x - dist;
        let rightp = x + dist;
        self.move_to(x, top)?;
        self.curve_to(leftp, top, left, up, left, y)?;
        self.curve_to(left, down, leftp, bottom, x, bottom)?;
        self.curve_to(rightp, bottom, right, down, right, y)?;
        self.curve_to(right, up, rightp, top, x, top)?;
        Ok(())
    }
    /// Stroke the current path.
    pub fn stroke(&mut self) -> io::Result<()> {
        writeln!(self.output, "S")
    }
    /// Close and stroke the current path.
    pub fn close_and_stroke(&mut self) -> io::Result<()> {
        writeln!(self.output, "s")
    }
    /// Fill the current path.
    pub fn fill(&mut self) -> io::Result<()> {
        writeln!(self.output, "f")
    }
    /// Get a FontRef for a specific font.
    pub fn get_font(&mut self, font: BuiltinFont) -> FontRef {
        use fontref::create_font_ref;
        let next_n = self.fonts.len();
        self.fonts
            .entry(font)
            .or_insert_with(|| {
                create_font_ref(
                    next_n,
                    font.get_encoding().clone(),
                    Arc::new(font.get_metrics()),
                )
            })
            .clone()
    }

    /// Create a text object.
    ///
    /// The contents of the text object is defined by the function
    /// render_text, by applying methods to the TextObject it gets as
    /// an argument.
    /// On success, return the value returned by render_text.
    pub fn text<F, T>(&mut self, render_text: F) -> io::Result<T>
    where
        F: FnOnce(&mut TextObject) -> io::Result<T>,
    {
        use textobject::create_text_object;
        writeln!(self.output, "BT")?;
        let result = render_text(&mut create_text_object(self.output))?;
        writeln!(self.output, "ET")?;
        Ok(result)
    }
    /// Utility method for placing a string of text.
    pub fn left_text(
        &mut self,
        x: f32,
        y: f32,
        font: BuiltinFont,
        size: f32,
        text: &str,
    ) -> io::Result<()> {
        let font = self.get_font(font);
        self.text(|t| {
            t.set_font(&font, size)?;
            t.pos(x, y)?;
            t.show(text)
        })
    }
    /// Utility method for placing a string of text.
    pub fn right_text(
        &mut self,
        x: f32,
        y: f32,
        font: BuiltinFont,
        size: f32,
        text: &str,
    ) -> io::Result<()> {
        let font = self.get_font(font);
        self.text(|t| {
            let text_width = font.get_width(size, text);
            t.set_font(&font, size)?;
            t.pos(x - text_width, y)?;
            t.show(text)
        })
    }
    /// Utility method for placing a string of text.
    pub fn center_text(
        &mut self,
        x: f32,
        y: f32,
        font: BuiltinFont,
        size: f32,
        text: &str,
    ) -> io::Result<()> {
        let font = self.get_font(font);
        self.text(|t| {
            let text_width = font.get_width(size, text);
            t.set_font(&font, size)?;
            t.pos(x - text_width / 2.0, y)?;
            t.show(text)
        })
    }

    /// Add an item for this page in the document outline.
    ///
    /// An outline item associates a name (contained in an ordered
    /// tree) with a location in the document.  The PDF standard
    /// supports several ways to specify an exact location on a page,
    /// but this implementation currently only supports linking to a
    /// specific page (the page that this Canvas is for).
    pub fn add_outline(&mut self, title: &str) {
        self.outline_items.push(OutlineItem::new(title));
    }

    /// Save the current graphics state.
    /// The caller is responsible for restoring it later.
    pub fn gsave(&mut self) -> io::Result<()> {
        writeln!(self.output, "q")
    }
    /// Restor the current graphics state.
    /// The caller is responsible for having saved it earlier.
    pub fn grestore(&mut self) -> io::Result<()> {
        writeln!(self.output, "Q")
    }
}
