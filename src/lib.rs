//! A library for creating pdf files.

//! # Example
//! ````no_run
//! use std::fs::File;
//! use pdf::{Pdf, FontSource};
//!
//! let mut file = File::create("foo.pdf").unwrap();
//! let mut document = Pdf::new(&mut file).unwrap();
//! let font = FontSource::Times_Roman;
//!
//! document.render_page(180.0, 240.0, |canvas| {
//!     canvas.center_text(90.0, 200.0, font, 24.0, "Hello World!")
//! });
//! ````

#[macro_use]
extern crate lazy_static;

extern crate time;

use std::io::{Seek, SeekFrom, Write, self};
use std::fmt;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs::File;
use std::sync::Arc;

mod fontmetrics;
pub use ::fontmetrics::FontMetrics;
use ::fontmetrics::get_builtin_metrics;

mod encoding;
pub use ::encoding::Encoding;
pub use ::encoding::WIN_ANSI_ENCODING;

mod outline;
pub use ::outline::OutlineItem;

/// The top-level object for writing a PDF.
pub struct Pdf<'a, W: 'a + Write + Seek> {
    output: &'a mut W,
    object_offsets: Vec<i64>,
    page_objects_ids: Vec<usize>,
    all_font_object_ids: HashMap<FontSource, usize>,
    outline_items: Vec<OutlineItem>,
    document_info: BTreeMap<String, String>
}

/// The "Base14" built-in fonts in PDF.
/// Underscores in these names are hyphens in the real names.
/// TODO Add a way to handle other fonts.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    fn write_object<'a, W: 'a + Write + Seek>(&self, pdf: &mut Pdf<'a, W>) -> io::Result<usize> {
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

/// A font ready to be used in a TextObject.
///
/// The way to get FontRef is to call `Canvas::get_font` with a
/// `FontSource`.  In PDF terms, a FontSource is a font dictionary,
/// and a FontRef is a name registered with a font source in the page
/// resources, like /F1.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontRef {
    n: usize,
    metrics: Arc<FontMetrics>
}

impl FontRef {
    /// Get the width of the given text in this font at given size.
    pub fn get_width(&self, size: f32, text: &str) -> f32 {
        size * self.get_width_raw(text) as f32 / 1000.0
    }

    /// Get the width of the given text in thousands of unit of text
    /// space.  This unit is what is used in some places internally in
    /// pdf files and in some methods on a `TextObject`.
    pub fn get_width_raw(&self, text: &str) -> u32 {
        let mut result = 0;
        for char in WIN_ANSI_ENCODING.encode_string(text) {
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

/// An visual area where content can be drawn (a page).
///
/// Provides methods for defining and stroking or filling paths, as
/// well as placing text objects.
pub struct Canvas<'a> {
    output: &'a mut Write,
    fonts: &'a mut HashMap<FontSource, FontRef>,
    outline_items: &'a mut Vec<OutlineItem>
}

/// A text object is where text is put on the canvas.
pub struct TextObject<'a> {
    output: &'a mut Write,
}

const ROOT_OBJECT_ID: usize = 1;
const PAGES_OBJECT_ID: usize = 2;

impl<'a, W: Write + Seek> Pdf<'a, W> {

    /// Create a new PDF document, writing to `output`.
    pub fn new(output: &'a mut W) -> io::Result<Pdf<'a, W>> {
        // FIXME: Find out the lowest version that contains the features we’re using.
        try!(output.write_all(b"%PDF-1.7\n%\xB5\xED\xAE\xFB\n"));
        Ok(Pdf {
            output: output,
            // Object ID 0 is special in PDF.
            // We reserve IDs 1 and 2 for the catalog and page tree.
            object_offsets: vec![-1, -1, -1],
            page_objects_ids: vec![],
            all_font_object_ids: HashMap::new(),
            outline_items: Vec::new(),
            document_info: BTreeMap::new(),
        })
    }
    /// Set metadata: the document's title.
    pub fn set_title(&mut self, title: &str) {
        self.document_info.insert("Title".to_string(), title.to_string());
    }
    /// Set metadata: the name of the person who created the document.
    pub fn set_author(&mut self, author: &str) {
        self.document_info.insert("Author".to_string(), author.to_string());
    }
    /// Set metadata: the subject of the document.
    pub fn set_subject(&mut self, subject: &str) {
        self.document_info.insert("Subject".to_string(), subject.to_string());
    }
    /// Set metadata: keywords associated with the document.
    pub fn set_keywords(&mut self, keywords: &str) {
        self.document_info.insert("Subject".to_string(), keywords.to_string());
    }
    /// Set metadata: If the document was converted to PDF from another
    /// format, the name of the conforming product that created the original
    /// document from which it was converted.
    pub fn set_creator(&mut self, creator: &str) {
        self.document_info.insert("Creator".to_string(), creator.to_string());
    }
    /// Set metadata: If the document was converted to PDF from another
    /// format, the name of the conforming product that converted it to PDF.
    pub fn set_producer(&mut self, producer: &str) {
        self.document_info.insert("Producer".to_string(), producer.to_string());
    }

    /// Return the current read/write position in the output file.
    fn tell(&mut self) -> io::Result<u64> {
        self.output.seek(SeekFrom::Current(0))
    }

    /// Create a new page in the PDF document.
    ///
    /// The page will be `width` x `height` points large, and the
    /// actual content of the page will be created by the function
    /// `render_contents` by applying drawing methods on the Canvas.
    pub fn render_page<F>(&mut self, width: f32, height: f32, render_contents: F) -> io::Result<()>
    where F: FnOnce(&mut Canvas) -> io::Result<()> {
        let (contents_object_id, content_length, fonts, outline_items) =
        try!(self.write_new_object(move |contents_object_id, pdf| {
            // Guess the ID of the next object. (We’ll assert it below.)
            try!(write!(pdf.output, "<<  /Length {} 0 R\n", contents_object_id + 1));
            try!(write!(pdf.output, ">>\n"));
            try!(write!(pdf.output, "stream\n"));

            let start = try!(pdf.tell());
            try!(write!(pdf.output, "/DeviceRGB cs /DeviceRGB CS\n"));
            let mut fonts : HashMap<FontSource, FontRef> = HashMap::new();
            let mut outline_items: Vec<OutlineItem> = Vec::new();
            try!(render_contents(&mut Canvas {
                output: pdf.output,
                fonts: &mut fonts,
                outline_items: &mut outline_items}));
            let end = try!(pdf.tell());

            try!(write!(pdf.output, "endstream\n"));
            Ok((contents_object_id, end - start, fonts, outline_items))
        }));
        try!(self.write_new_object(|length_object_id, pdf| {
            assert!(length_object_id == contents_object_id + 1);
            write!(pdf.output, "{}\n", content_length)
        }));

        let mut font_object_ids : HashMap<FontRef, usize> = HashMap::new();
        for (src, r) in &fonts {
            if let Some(&object_id) = self.all_font_object_ids.get(&src) {
                font_object_ids.insert(r.clone(), object_id);
            } else {
                let object_id = try!(src.write_object(self));
                font_object_ids.insert(r.clone(), object_id);
                self.all_font_object_ids.insert(src.clone(), object_id);
           }
        }
        let page_object_id = try!(self.write_new_object(|page_object_id, pdf| {
            try!(write!(pdf.output, "<<  /Type /Page\n"));
            try!(write!(pdf.output, "    /Parent {} 0 R\n", PAGES_OBJECT_ID));
            try!(write!(pdf.output, "    /Resources << /Font << "));
            for (r, object_id) in &font_object_ids {
                try!(write!(pdf.output, "{} {} 0 R ", r, object_id));
            }
            try!(write!(pdf.output, ">> >>\n"));
            try!(write!(pdf.output, "    /MediaBox [ 0 0 {} {} ]\n", width, height));
            try!(write!(pdf.output, "    /Contents {} 0 R\n", contents_object_id));
            try!(write!(pdf.output, ">>\n"));
            Ok(page_object_id)
        }));
        // Take the outline_items from this page, mark them with the page ref,
        // and save them for the document outline.
        for i in &outline_items {
            let mut item = i.clone();
            item.set_page(page_object_id);
            self.outline_items.push(item);
        }
        self.page_objects_ids.push(page_object_id);
        Ok(())
    }

    fn write_new_object<F, T>(&mut self, write_content: F) -> io::Result<T>
    where F: FnOnce(usize, &mut Pdf<W>) -> io::Result<T> {
        let id = self.object_offsets.len();
        let (result, offset) = try!(self.write_object(id, |pdf| write_content(id, pdf)));
        self.object_offsets.push(offset);
        Ok(result)
    }

    fn write_object_with_id<F, T>(&mut self, id: usize, write_content: F) -> io::Result<T>
    where F: FnOnce(&mut Pdf<W>) -> io::Result<T> {
        assert!(self.object_offsets[id] == -1);
        let (result, offset) = try!(self.write_object(id, write_content));
        self.object_offsets[id] = offset;
        Ok(result)
    }

    fn write_object<F, T>(&mut self, id: usize, write_content: F) -> io::Result<(T, i64)>
    where F: FnOnce(&mut Pdf<W>) -> io::Result<T> {
        // `as i64` here would only overflow for PDF files bigger than 2**63 bytes
        let offset = try!(self.tell()) as i64;
        try!(write!(self.output, "{} 0 obj\n", id));
        let result = try!(write_content(self));
        try!(write!(self.output, "endobj\n"));
        Ok((result, offset))
    }

    /// Write out the document trailer.
    /// The trailer consists of the pages object, the root object,
    /// the xref list, the trailer object and the startxref position.
    pub fn finish(mut self) -> io::Result<()> {
        try!(self.write_object_with_id(PAGES_OBJECT_ID, |pdf| {
            try!(write!(pdf.output, "<<  /Type /Pages\n"));
            try!(write!(pdf.output, "    /Count {}\n", pdf.page_objects_ids.len()));
            try!(write!(pdf.output, "    /Kids [ "));
            for &page_object_id in &pdf.page_objects_ids {
                try!(write!(pdf.output, "{} 0 R ", page_object_id));
            }
            try!(write!(pdf.output, "]\n"));
            try!(write!(pdf.output, ">>\n"));
            Ok(())
        }));
        let document_info_id =
            if !self.document_info.is_empty() {
                let info = self.document_info.clone();
                try!(self.write_new_object(|page_object_id, pdf| {
                    try!(write!(pdf.output, "<<"));
                    for (key, value) in info {
                        try!(write!(pdf.output, " /{} ({})\n", key, value));
                    }
                    if let Ok(now) = time::strftime("%Y%m%d%H%M%S%z",
                                                    &time::now()) {
                        try!(write!(pdf.output, " /CreationDate (D:{})", now));
                        try!(write!(pdf.output, " /ModDate (D:{})", now));
                    }
                    try!(write!(pdf.output, ">>\n"));
                    Ok(Some(page_object_id))
                }))
            } else { None };

        let outlines_id =
            if !self.outline_items.is_empty() {
                let parent_id = self.object_offsets.len();
                self.object_offsets.push(-1);
                let count = self.outline_items.len();
                let mut first_id = 0;
                let mut last_id = 0;
                let items = self.outline_items.clone();
                for (i, item) in items.iter().enumerate() {
                    let (is_first, is_last) = (i == 0, i == count -1);
                    let id = try!(self.write_new_object(|object_id, pdf| {
                        item.write_dictionary(
                            pdf.output, parent_id,
                            if is_first {None} else {Some(object_id-1)},
                            if is_last {None} else {Some(object_id+1)})
                            .and(Ok(object_id))}));
                    if is_first {
                        first_id = id;
                    }
                    if is_last {
                        last_id = id;
                    }
                }
                try!(self.write_object_with_id(parent_id, |pdf| {
                    try!(write!(pdf.output, "<< /Type /Outlines\n"));
                    try!(write!(pdf.output, "/First {} 0 R\n", first_id));
                    try!(write!(pdf.output, "/Last {} 0 R\n", last_id));
                    try!(write!(pdf.output, "/Count {}\n", count));
                    write!(pdf.output, ">>\n")
                }));
                Some(parent_id)
            } else {
                None
            };

        try!(self.write_object_with_id(ROOT_OBJECT_ID, |pdf| {
            try!(write!(pdf.output, "<<  /Type /Catalog\n"));
            try!(write!(pdf.output, "    /Pages {} 0 R\n", PAGES_OBJECT_ID));
            if let Some(outlines_id) = outlines_id {
                try!(write!(pdf.output, "/Outlines {} 0 R\n", outlines_id));
            }
            try!(write!(pdf.output, ">>\n"));
            Ok(())
        }));
        let startxref = try!(self.tell());
        try!(write!(self.output, "xref\n"));
        try!(write!(self.output, "0 {}\n", self.object_offsets.len()));
        // Object 0 is special
        try!(write!(self.output, "0000000000 65535 f \n"));
        // Use [1..] to skip object 0 in self.object_offsets.
        for &offset in &self.object_offsets[1..] {
            assert!(offset >= 0);
            try!(write!(self.output, "{:010} 00000 n \n", offset));
        }
        try!(write!(self.output, "trailer\n"));
        try!(write!(self.output, "<<  /Size {}\n", self.object_offsets.len()));
        try!(write!(self.output, "    /Root {} 0 R\n", ROOT_OBJECT_ID));
        if let Some(id) = document_info_id {
            try!(write!(self.output, "    /Info {} 0 R\n", id));
        }
        try!(write!(self.output, ">>\n"));
        try!(write!(self.output, "startxref\n"));
        try!(write!(self.output, "{}\n", startxref));
        try!(write!(self.output, "%%EOF\n"));
        Ok(())
    }
}

impl<'a> Canvas<'a> {
    /// Append a closed rectangle with a corern at (x, y) and
    /// extending width × height to the to the current path.
    pub fn rectangle(&mut self, x: f32, y: f32, width: f32, height: f32)
                     -> io::Result<()> {
        write!(self.output, "{} {} {} {} re\n", x, y, width, height)
    }
    /// Set the line width in the graphics state
    pub fn set_line_width(&mut self, w: f32) -> io::Result<()> {
        write!(self.output, "{} w\n", w)
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
    /// Append a straight line from (x1, y1) to (x2, y2) to the current path.
    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32)
                -> io::Result<()> {
        try!(self.move_to(x1, y1));
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
    /// Add an Bézier curve from the current point to (x3, y3) with
    /// (x1, y1) and (x2, y2) as Bézier controll points.
    pub fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32,
                    x3: f32, y3: f32) -> io::Result<()> {
        write!(self.output, "{} {} {} {} {} {} c\n", x1, y1, x2, y2, x3, y3)
    }
    /// Add a circle approximated by four cubic Bézier curves to the
    /// current path.  Based on
    /// http://spencermortensen.com/articles/bezier-circle/
    pub fn circle(&mut self, x: f32, y: f32, r: f32) -> io::Result<()> {
        let t = y - r;
        let b = y + r;
        let left = x - r;
        let right = x + r;
        let c = 0.551915024494;
        let leftp = x - (r * c);
        let rightp = x + (r * c);
        let tp = y - (r * c);
        let bp = y + (r * c);
        try!(self.move_to(x, t));
        try!(self.curve_to(leftp, t, left, tp, left, y));
        try!(self.curve_to(left, bp, leftp, b, x, b));
        try!(self.curve_to(rightp, b, right, bp, right, y));
        try!(self.curve_to(right, tp, rightp, t, x, t));
        Ok(())
    }
    /// Stroke the current path.
    pub fn stroke(&mut self) -> io::Result<()> {
        write!(self.output, "s\n")
    }
    /// Fill the current path.
    pub fn fill(&mut self) -> io::Result<()> {
        write!(self.output, "f\n")
    }
    /// Get a FontRef for a specific font.
    pub fn get_font(&mut self, font: FontSource) -> FontRef {
        if let Some(r) = self.fonts.get(&font) {
            return r.clone();
        }
        let n = self.fonts.len();
        let r = FontRef { n: n, metrics: Arc::new(font.get_metrics().unwrap()) };
        self.fonts.insert(font, r.clone());
        r
    }
    /// Create a text object.
    ///
    /// The contents of the text object is defined by the function
    /// render_text, by applying methods to the TextObject it gets as
    /// an argument.
    pub fn text<F, T>(&mut self, render_text: F) -> io::Result<T>
        where F: FnOnce(&mut TextObject) -> io::Result<T> {
            try!(write!(self.output, "BT\n"));
            let result =
                try!(render_text(&mut TextObject { output: self.output }));
            try!(write!(self.output, "ET\n"));
            Ok(result)
        }
    /// Utility method for placing a string of text.
    pub fn right_text(&mut self, x: f32, y: f32, font: FontSource, size: f32,
                      text: &str) -> io::Result<()> {
        let font = self.get_font(font);
        self.text(|t| {
            let text_width = font.get_width(size, text);
            try!(t.set_font(&font, size));
            try!(t.pos(x - text_width, y));
            t.show(text)
        })
    }
    /// Utility method for placing a string of text.
    pub fn center_text(&mut self, x: f32, y: f32, font: FontSource, size: f32,
                       text: &str) -> io::Result<()> {
        let text_width = font.get_width(size, text);
        let font = self.get_font(font);
        self.text(|t| {
            try!(t.set_font(&font, size));
            try!(t.pos(x - text_width / 2.0, y));
            t.show(text)
        })
    }
    /// Add an item for this page in the document outline.
    pub fn add_outline(&mut self, title: &str) {
        self.outline_items.push(OutlineItem::new(title));
    }
}

impl<'a> TextObject<'a> {
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
