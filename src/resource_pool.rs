use font::Font;
use objects::PdfObject;
use objects::PdfReference;
use std::collections::BTreeMap;

/// A pool for collecting resources into a central location
#[derive(Debug)]
pub struct ResourcePool {
    fonts: BTreeMap<FontId, Font>,
    next_font_id: FontId
}

/// A map from IDs to References.
/// This is used internally while outputting the document.
#[derive(Debug)]
pub struct ResourceReferences {
    fonts: BTreeMap<FontId, PdfReference>,
    max_ref: PdfReference
}

/// Font IDs
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct FontId(u32);

impl ResourcePool {
    /// Create a new ResourcePool
    pub fn new() -> ResourcePool {
        ResourcePool {
            fonts: BTreeMap::new(),
            next_font_id: FontId(0),
        }
    }

    pub fn insert_font(&mut self, name: &str) -> FontId {
        let font_id = self.next_font_id;
        let font = Font::new(font_id.0, name);
        self.fonts.insert(font_id, font);
        self.next_font_id.0 += 1;
        font_id
    }

    /// Append the resources to the body
    pub fn append_to_body(&self,
                       builder: &mut BTreeMap<PdfReference, PdfObject>,
                       mut next_key: PdfReference)
                       -> ResourceReferences {
        let mut res_refs = ResourceReferences::new();
        for (id, font) in &self.fonts {
            res_refs.insert_font(*id, next_key);
            builder.insert(next_key, font.to_dictionary().into());
            next_key = next_key.next();
        }
        res_refs
    }
}

impl ResourceReferences {
    /// Create a new ResourceReferences.
    fn new() -> ResourceReferences {
        ResourceReferences {
            fonts: BTreeMap::new(),
            max_ref: PdfReference::new(0)
        }
    }

    fn insert_font(&mut self, key: FontId, pdf_ref: PdfReference) {
        self.fonts.insert(key, pdf_ref);
    }

    pub fn get_font_ref(&self, key: FontId) -> Option<PdfReference> {
        self.fonts.get(&key).map(|val| *val)
    }

    pub fn num_refs(&self) -> u32 {
        self.fonts.len() as u32
    }
}

impl FontId {
    /// Ease of use function, getting the name of the font.
    pub fn get_name(&self) -> String {
        format!("F{}", self.0)
    }
}