use objects::PdfArray;
use objects::PdfDictionary;
use objects::PdfInteger;
use objects::PdfName;
use objects::PdfObject;
use objects::PdfReference;
use page::Page;
use rectangle::Rectangle;
use resource_pool::ResourceReferences;

use std::collections::BTreeMap;

/// A tree of pages
#[derive(Debug)]
pub struct Pages {
    kids: Vec<Page>,
    crop_box: Option<Rectangle>,
    media_box: Option<Rectangle>,
}

impl Pages {
    /// Create a new Pages
    pub fn new() -> Pages {
        Pages {
            kids: Vec::new(),
            crop_box: None,
            media_box: None,
        }
    }

    /// Set the dimensions that children inherit
    pub fn set_dimensions(&mut self, rect: Rectangle) {
        self.crop_box = Some(rect);
        self.media_box = Some(rect);
    }

    /// Add a new page, returning a mutable reference
    pub fn append_page(&mut self) -> &mut Page {
        let page = Page::new();
        self.kids.push(page);
        let index = self.kids.len() - 1;
        self.kids.get_mut(index).unwrap()
    }
}

impl Pages {
    fn to_dictionary(&self, kid_refs: Vec<PdfReference>) -> PdfDictionary {
        let mut builder = PdfDictionary::new();
        builder.set("Type", PdfName::new("Pages"));
        builder.set("Kids", PdfArray::from(kid_refs));
        builder.set("Count", PdfInteger::new(self.kids.len() as i64));
        if let Some(rect) = self.crop_box {
            builder.set("CropBox", rect.to_array());
        }
        if let Some(rect) = self.media_box {
            builder.set("MediaBox", rect.to_array());
        }
        // TODO: The rest
        builder
    }

    /// Modify the builder in place, adding pages recursively.
    /// Returns the last used reference key.
    pub fn append_to_body(&self,
                       self_ref: PdfReference,
                       builder: &mut BTreeMap<PdfReference, PdfObject>,
                       res: &ResourceReferences)
                       -> PdfReference {
        let mut kid_ref = self_ref;
        let mut kid_refs = Vec::with_capacity(self.kids.len());
        for kid in &self.kids {
            let cont_ref = kid_ref.next();
            kid_ref = cont_ref.next();
            kid_refs.push(kid_ref);
            kid.append_to_body(self_ref, kid_ref, cont_ref, builder, res);
        }
        builder.insert(self_ref, self.to_dictionary(kid_refs).into());
        kid_ref
    }
}
