use objects::PdfDictionary;
use objects::PdfName;

/// FIXME
#[derive(Debug)]
pub struct Font {
    id: u32,
    basefont: String,
}

impl Font {
    /// Create a new Font
    pub fn new(id: u32, name: &str) -> Font {
        Font {
            id: id,
            basefont: name.to_owned()
        }
    }

    /// Create the PdfDictionary representing this font.
    pub fn to_dictionary(&self) -> PdfDictionary {
        let mut builder = PdfDictionary::new();
        builder.set("Type", PdfName::new("Font"));
        builder.set("Subtype", PdfName::new("Type1"));
        builder.set("Name", PdfName::new(format!("F{}", self.id)));
        builder.set("BaseFont", PdfName::new(self.basefont.clone()));
        builder
    }
}