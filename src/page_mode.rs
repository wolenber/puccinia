/// When the file is open, should outlines or thumbs be visible?
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum PageMode {
    /// Neither Outlines nor Thumbs are visible.
    None,
    /// Outlines are visible when the document is opened.
    Outlines,
    /// Thumbs are visible when the document is opened.
    Thumbs,
}
