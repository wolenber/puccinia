use objects::PdfDictionary;
use objects::PdfName;
use objects::PdfObject;
use objects::PdfReference;
use outlines::Outlines;
use page_mode::PageMode;
use pages::Pages;
use resource_pool::ResourcePool;

use std::collections::BTreeMap;

/// The root of the document structure
#[derive(Debug)]
pub struct Catalog {
    pages: Pages,
    outlines: Option<Outlines>,
    page_mode: PageMode,
}

impl Catalog {
    /// Create a new Catalog
    pub fn new() -> Catalog {
        Catalog {
            pages: Pages::new(),
            outlines: None,
            page_mode: PageMode::None
        }
    }

    /// Return a mutable references to the pages object
    pub fn pages_mut(&mut self) -> &mut Pages {
        &mut self.pages
    }

    fn to_dictionary(&self, pages: PdfReference) -> PdfDictionary {
        let mut builder = PdfDictionary::new();
        builder.set("Type", PdfName::new("Catalog"));
        builder.set("Pages", pages);
        builder
    }

    /// Create the body of the pdf file
    pub fn create_body(&self, resources: &ResourcePool) -> BTreeMap<PdfReference, PdfObject> {
        let mut builder = BTreeMap::new();

        let self_ref = PdfReference::new(1);
        let res_refs = resources.append_to_body(&mut builder, self_ref.next());
        let pages_ref = PdfReference::new(self_ref.id + res_refs.num_refs() + 1);

        builder.insert(self_ref, self.to_dictionary(pages_ref).into());
        self.pages.append_to_body(pages_ref, &mut builder, &res_refs);
        builder
    }
}
