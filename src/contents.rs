use objects::PdfStream;
use objects::PdfString;
use output::Output;
use resource_pool::FontId;

/// FIXME
#[derive(Debug)]
pub struct Contents {
    repr: PdfStream
}

impl Contents {
    /// Create a new Contents
    pub fn new() -> Contents {
        Contents {
            repr: PdfStream::new()
        }
    }

    /// Use this as a TextContents
    pub fn as_text_contents<'a>(&'a mut self) -> TextContents<'a> {
        TextContents::new(&mut self.repr)
    }

    /// Turn this into an output friendly string
    pub fn get_repr(&self) -> &PdfStream {
        &self.repr
    }
}

/// FIXME
#[derive(Debug)]
pub struct TextContents<'repr> {
    repr: &'repr mut PdfStream
}

impl <'repr> TextContents<'repr> {
    /// Called when the TextContents is created
    fn new(stream: &'repr mut PdfStream) -> TextContents {
        stream.append_line("BT");
        TextContents {
            repr: stream
        }
    }

    /// Called to change the current font in the text stream
    pub fn set_font(&mut self, fnt: FontId, size: f64) -> &mut TextContents<'repr> {
        self.repr.append_line(
                &format!("/{} {} Tf", fnt.get_name(), size));
        self
    }

    /// Displace text by (dx, dy)
    pub fn displace(&mut self, dx: f64, dy: f64) -> &mut TextContents<'repr> {
        self.repr.append_line(
                &format!("{} {} Td", dx, dy));
        self
    }

    /// Show a string of text
    pub fn show_text(&mut self, text: &str) -> &mut TextContents<'repr> {
        self.repr.append_line(
                &format!("{} Tj", PdfString::new(text).output()));
        self
    }
}

impl <'repr> Drop for TextContents<'repr> {
    fn drop(&mut self) {
        self.repr.append_line("ET");
    }
}