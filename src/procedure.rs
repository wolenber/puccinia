use objects::PdfArray;
use objects::PdfName;

// A proc is a procedure available while rendering a page.
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Procedure {
    /// Required by any non-blank pages
    Pdf,
    /// Required for text
    Text,
    /// Black and white images
    ImageB,
    /// Color images
    ImageC,
    /// Indexed images
    ImageI,
}

impl Procedure {
    /// Create an owned string from the procedure
    /// HACK Should probably return a static str, then call .to_owned()
    pub fn to_string(&self) -> String {
        use self::Procedure::*;
        match *self {
            Pdf => "PDF".to_owned(),
            Text => "Text".to_owned(),
            ImageB => "ImageB".to_owned(),
            ImageC => "ImageC".to_owned(),
            ImageI => "ImageI".to_owned(),
        }
    }
}

/// Sets of procedures
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ProcSet {
    /// The procedure for a page containing nothing
    None,
    /// The procedure text for self
    TextOnly,
}

impl ProcSet {
    /// Convert the ProcSet to a useable PdfArray
    pub fn to_array(&self) -> PdfArray {
        let vec = match *self {
            ProcSet::None => vec![ Procedure::Pdf ],
            ProcSet::TextOnly => vec![ Procedure::Pdf, Procedure::Text ],
        }.iter().map(|p| PdfName::new(p.to_string())).collect();
        PdfArray::from(vec)
    }
}
