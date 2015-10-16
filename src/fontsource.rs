use std::io::{Seek, Write, self};
use std::fs::File;
use ::encoding::WIN_ANSI_ENCODING;
use ::fontmetrics::FontMetrics;
use ::fontmetrics::get_builtin_metrics;
use ::Pdf;

/// The "Base14" built-in fonts in PDF.
/// Underscores in these names are hyphens in the real names.
/// TODO Add a way to handle other fonts.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum FontSource {
    Courier,
    Courier_Bold,
    Courier_Oblique,
    Courier_BoldOblique,
    Helvetica,
    Helvetica_Bold,
    Helvetica_Oblique,
    Helvetica_BoldOblique,
    Times_Roman,
    Times_Bold,
    Times_Italic,
    Times_BoldItalic,
    Symbol,
    ZapfDingbats
}

impl FontSource {
    pub fn write_object<'a, W: 'a + Write + Seek>(&self, pdf: &mut Pdf<'a, W>) -> io::Result<usize> {
        // Note: This is enough for a Base14 font, other fonts will
        // require a stream for the actual font, and probably another
        // object for metrics etc
        pdf.write_new_object(|font_object_id, pdf| {
            try!(write!(pdf.output,
                        "<< /Type /Font /Subtype /Type1 /BaseFont /{} /Encoding /WinAnsiEncoding >>\n",
                        self.pdf_name()));
            Ok(font_object_id)
        })
    }

    /// Get the PDF name of this font.
    /// # Examples
    /// ```
    /// use pdf::FontSource;
    /// assert_eq!("Times-Roman", FontSource::Times_Roman.pdf_name());
    /// ```
    pub fn pdf_name(&self) -> String {
        format!("{:?}", self).replace("_", "-")
    }

    /// Get the width of a string in this font at given size.
    ///
    /// # Examples
    /// ```
    /// use pdf::FontSource;
    /// assert_eq!(62.004, FontSource::Helvetica.get_width(12.0, "Hello World"));
    /// assert_eq!(60.0, FontSource::Courier.get_width(10.0, "0123456789"));
    /// ```
    pub fn get_width(&self, size: f32, text: &str) -> f32 {
        size * self.get_width_raw(text) as f32 / 1000.0
    }

    /// Get the width of a string in thousands of unit of text space.
    /// This unit is what is used in some places internally in pdf files.
    ///
    /// # Examples
    /// ```
    /// use pdf::FontSource;
    /// assert_eq!(5167, FontSource::Helvetica.get_width_raw("Hello World"));
    /// assert_eq!(600, FontSource::Courier.get_width_raw("A"));
    /// ```
    pub fn get_width_raw(&self, text: &str) -> u32 {
        if let Ok(metrics) = self.get_metrics() {
            let mut result = 0;
            for char in WIN_ANSI_ENCODING.encode_string(text) {
                result += metrics.get_width(char).unwrap_or(100) as u32;
            }
            result
        } else {
            0
        }
    }

    /// Get the font metrics for font.
    pub fn get_metrics(&self) -> io::Result<FontMetrics> {
        if let Some(result) = get_builtin_metrics(&self.pdf_name()) {
            return Ok(result);
        }
        // TODO Non-builtin metrics wont be found here, use some search path.
        let filename = format!("data/{}.afm", self.pdf_name());
        println!("Reading metrics {}", filename);
        match File::open(&filename) {
            Ok(file) => FontMetrics::parse(file),
            Err(e) => Err(e)
        }
    }
}
