pub use encoding::WIN_ANSI_ENCODING;
use std::io::{self, Write};

/// An item in the document outline.
///
/// An OutlineItem associates a name (contained in an ordered tree)
/// with a location in the document.  The PDF standard supports
/// several ways to specify an exact location on a page, but this
/// implementation currently only supports linking to a specific page.
///
/// To actually create an OutlineItem in a meaningful way, please
/// use `Canvas::add_outline`.
#[derive(Clone)]
pub struct OutlineItem {
    title: String,
    page_id: Option<usize>,
}

impl OutlineItem {
    pub fn new(title: &str) -> OutlineItem {
        OutlineItem {
            title: title.to_string(),
            page_id: None,
        }
    }

    pub fn set_page(&mut self, page_id: usize) {
        self.page_id = Some(page_id)
    }

    pub fn write_dictionary(
        &self,
        output: &mut Write,
        parent_id: usize,
        prev: Option<usize>,
        next: Option<usize>,
    ) -> io::Result<()> {
        output.write_all(b"<< /Title (")?;
        output.write_all(&WIN_ANSI_ENCODING.encode_string(&self.title))?;
        output.write_all(b")\n")?;
        writeln!(output, "/Parent {} 0 R", parent_id)?;
        if let Some(id) = prev {
            writeln!(output, "/Prev {} 0 R", id)?;
        }
        if let Some(id) = next {
            writeln!(output, "/Next {} 0 R", id)?;
        }
        if let Some(id) = self.page_id {
            writeln!(output, "/Dest [{} 0 R /XYZ null null null]", id)?;
        }
        writeln!(output, ">>")
    }
}
