use super::PdfObject;

use output::Output;

/// A PDF name
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct PdfName(String);

impl PdfName {
    /// Create a new PDF name
    pub fn new<T>(s: T) -> PdfName
            where T: Into<String> {
        let s = s.into();
        assert!(&s != "");
        PdfName(s)
    }

    /// Return this name as a string slice, without prefix
    pub fn as_str_slice(&self) -> &str {
        &self.0
    }
}

impl_into_from_case! { PdfObject : PdfObject::Name => PdfName }

impl Output for PdfName {
    fn output(&self) -> String {
        format!("/{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::PdfName;
    use output::Output;

    #[test]
    fn output() {
        const EXPECTED: &'static str = "/Foo";
        let actual = PdfName::new("Foo").output();
        assert_eq!(actual, EXPECTED);
    }
    
    #[test]
    #[should_panic]
    fn empty_name() {
        PdfName::new("");
    }
}
