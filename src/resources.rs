use objects::PdfDictionary;
use procedure::ProcSet;
use resource_pool::FontId;
use resource_pool::ResourceReferences;

/// FIXME
#[derive(Debug)]
pub struct Resources {
    pub fonts: Vec<FontId>,
    pub proc_set: ProcSet,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            fonts: Vec::new(),
            proc_set: ProcSet::None,
        }
    }

    pub fn to_dictionary(&self,
                     res: &ResourceReferences)
                     -> PdfDictionary {
        let mut builder = PdfDictionary::new();
        let fonts = {
            let mut buf = PdfDictionary::new();
            for &id in &self.fonts {
                buf.set(&id.get_name(), res.get_font_ref(id).unwrap());
            }
            buf
        };
        builder.set("Font", fonts);
        builder.set("ProcSet", self.proc_set.to_array());
        builder
    }
}
