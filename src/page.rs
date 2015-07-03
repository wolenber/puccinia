use contents::Contents;
use objects::PdfDictionary;
use objects::PdfName;
use objects::PdfObject;
use objects::PdfReference;
use procedure::ProcSet;
use rectangle::Rectangle;
use resource_pool::FontId;
use resource_pool::ResourceReferences;
use resources::Resources;
use rotation::Rotation;

use std::collections::BTreeMap;

/// A page
#[derive(Debug)]
pub struct Page {
    crop_box: Option<Rectangle>,
    media_box: Option<Rectangle>,
    resources: Resources,
    contents: Contents,
    rotation: Option<Rotation>,
}

impl Page {
    /// Create a new Page
    pub fn new() -> Page {
        Page {
            crop_box: None,
            media_box: None,
            resources: Resources::new(),
            contents: Contents::new(),
            rotation: None
        }
    }

    /// Add font to the resources of the page
    pub fn with_font(&mut self, font: FontId) -> &mut Page {
        self.resources.fonts.push(font);
        self
    }

    /// Set the ProcSet of the page.
    pub fn with_procset(&mut self, procs: ProcSet) -> &mut Page {
        self.resources.proc_set = procs;
        self
    }

    /// Returns a mutable reference to the contents
    pub fn use_contents(&mut self) -> &mut Contents {
        &mut self.contents
    }
}

impl Page {
    fn to_dictionary(&self,
                     parent: PdfReference,
                     contents: PdfReference,
                     res: &ResourceReferences)
                     -> PdfDictionary {
        let mut builder = PdfDictionary::new();
        builder.set("Type", PdfName::new("Page"));
        builder.set("Parent", parent);
        builder.set("Contents", contents);
        builder.set("Resources", self.resources.to_dictionary(res));
        builder
    }

    /// Modify the builder in place, adding pages recursively.
    /// Returns the last used reference key.
    pub fn append_to_body(&self,
                       parent_ref: PdfReference,
                       self_ref: PdfReference,
                       contents_ref: PdfReference,
                       builder: &mut BTreeMap<PdfReference, PdfObject>,
                       res: &ResourceReferences) {
        builder.insert(self_ref, self.to_dictionary(parent_ref, contents_ref, res).into());
        builder.insert(contents_ref, self.contents.get_repr().clone().into());
    }
}
