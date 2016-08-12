//! Types for representing details in the graphics state.

/// Line join styles, as described in section 8.4.3.4 of the PDF
/// specification.
pub enum JoinStyle {
    /// The outer edges continues until they meet.
    Miter,
    /// The lines are joined by a circle of line-width diameter.
    Round,
    /// End the lines as with `CapStyle::Butt` and fill the resulting
    /// gap with a triangle.
    Bevel,
}

/// Line cap styles, as described in section 8.4.3.4 of the PDF
/// specification.
pub enum CapStyle {
    /// Truncate the line squarely through the endpoint.
    Butt,
    /// Include a circle of line-width diameter around the endpoint.
    Round,
    /// Include a square around the endpoint, so the line continues for half
    /// a line-width through the endpoint.
    ProjectingSquare,
}

/// Any color (or grayscale) value that this library can make PDF represent.
pub enum Color {
    #[doc(hidden)]
    RGB {
        red: u8,
        green: u8,
        blue: u8,
    },
    #[doc(hidden)]
    Gray {
        gray: u8,
    }
}

impl Color {

    /// Return a color from a RGB colorspace.

    /// # Example
    /// ````
    /// let white  = Color::rgb(255, 255, 255);
    /// let black  = Color::rgb(0, 0, 0);
    /// let red    = Color::rgb(255, 0, 0);
    /// let yellow = Color::rgb(255, 255, 0);
    /// ````
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color::RGB { red: red, green: green, blue: blue }
    }

    /// Return a grayscale color value.

    /// # Example
    /// ````
    /// let white = Color::gray(255);
    /// let gray  = Color::gray(128);
    /// ````
    pub fn gray(gray: u8) -> Self {
        Color::Gray { gray: gray }
    }

}
