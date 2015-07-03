use catalog::Catalog;
use objects::PdfDictionary;
use objects::PdfInteger;
use objects::PdfReference;
use output::Output;
use page::Page;
use pages::Pages;
use resource_pool::FontId;
use resource_pool::ResourcePool;
use version::Version;

use std::collections::BTreeMap;

/// A PDF document
#[derive(Debug)]
pub struct Document {
    version: Version,
    catalog: Catalog,
    resource_pool: ResourcePool,
}

impl Document {
    /// Create a new Document
    pub fn new() -> Document {
        Document {
            version: Version::V10,
            catalog: Catalog::new(),
            resource_pool: ResourcePool::new(),
        }
    }

    /// Return a mutable reference to the root Pages object
    pub fn pages_mut(&mut self) -> &mut Pages {
        self.catalog.pages_mut()
    }

    /// Create a new Page, add it to the end of the root Pages, and return a mutable reference.
    pub fn append_page(&mut self) -> &mut Page {
        self.pages_mut().append_page()
    }

    /// Include one of the 14 guaranteed fonts
    pub fn include_base_font(&mut self, name: &str) -> FontId {
        self.resource_pool.insert_font(name)
    }
}

impl Output for Document {
    fn output(&self) -> String {
        let mut builder = String::new();
        let mut xrefs = BTreeMap::new();

        // Output version number
        builder.push_str(&format!("%{}\n\n", self.version.output()));

        // Output body
        let body = self.catalog.create_body(&self.resource_pool);
        for (refkey, object) in &body {
            xrefs.insert(refkey.clone(), builder.len());
            builder.push_str(&format!("{} {} obj\n", refkey.id, refkey.gen));
            builder.push_str(&format!("{}\n", object.output()));
            builder.push_str("endobj\n\n");
        }

        // Output xrefs
        let xref_offset = builder.len();
        let xref_num_entries = body.len() + 1;
        builder.push_str("xref\n");
        builder.push_str(&format!("0 {}\n", xref_num_entries));
        builder.push_str("0000000000 65535 f \n");
        for (_refkey, offset) in &xrefs {
            builder.push_str(&format!("{:0>10} {:0>5} n \n", offset, 0));
        }

        // Output trailer
        builder.push_str("\ntrailer\n");
        let mut trailer = PdfDictionary::new();
        trailer.set("Size", PdfInteger::new(xref_num_entries as i64));
        trailer.set("Root", PdfReference::new(1));
        builder.push_str(&trailer.output());
        builder.push_str("\nstartxref\n");
        builder.push_str(&xref_offset.to_string());
        builder.push_str("\n%%EOF");
        builder
    }
}
