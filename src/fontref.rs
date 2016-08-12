use std::fmt;
use std::sync::Arc;

use fontmetrics::FontMetrics;
use encoding::Encoding;

/// A font ready to be used in a TextObject.
///
/// The way to get FontRef is to call `Canvas::get_font` with a
/// `FontSource`.  In PDF terms, a FontSource is a font dictionary,
/// and a FontRef is a name registered with a font source in the page
/// resources, like /F1.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontRef {
    n: usize,
    encoding: Encoding,
    metrics: Arc<FontMetrics>,
}

impl FontRef {
    pub fn new(n: usize, encoding: Encoding, metrics: Arc<FontMetrics>) -> FontRef {
        FontRef {
            n: n,
            encoding: encoding,
            metrics: metrics,
        }
    }

    pub fn get_encoding(&self) -> Encoding {
        self.encoding.clone()
    }

    /// Get the width of the given text in this font at given size.
    pub fn get_width(&self, size: f32, text: &str) -> f32 {
        size * self.get_width_raw(text) as f32 / 1000.0
    }

    /// Get the width of the given text in thousands of unit of text
    /// space.  This unit is what is used in some places internally in
    /// pdf files and in some methods on a `TextObject`.
    pub fn get_width_raw(&self, text: &str) -> u32 {
        let mut result = 0;
        for char in self.encoding.encode_string(text) {
            result += self.metrics.get_width(char).unwrap_or(100) as u32;
        }
        result
    }
}

impl fmt::Display for FontRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/F{}", self.n)
    }
}
