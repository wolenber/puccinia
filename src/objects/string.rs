use super::PdfObject;

use output::Output;

/// A PDF string
#[derive(Debug)]
pub struct PdfString(String);

impl PdfString {
    /// Create a new PdfString
    pub fn new<T>(s: T) -> PdfString
            where T: Into<String> {
        let s = s.into();
        PdfString(s)
    }
}

impl_into_from_case! { PdfObject : PdfObject::String => PdfString }

impl Output for PdfString {
    fn output(&self) -> String {
        let builder = self.0.clone();
        let builder = builder.replace("\\", "\\\\");
        let builder = builder.replace("\n", "\\\n");
        let builder = builder.replace("\r", "\\\r");
        let builder = builder.replace("\t", "\\\t");
        let builder = builder.replace("(", "\\(");
        let builder = builder.replace(")", "\\)");
        format!("({})", builder)
    }
}


#[cfg(test)]
mod test {
    use super::PdfString;
    use output::Output;

    #[test]
    fn output() {
        const EXPECTED: &'static str = "(Foo bar)";
        let actual = PdfString::new("Foo bar").output();
        assert_eq!(actual, EXPECTED);
    }
}
