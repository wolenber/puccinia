use super::PdfObject;

use output::Output;

/// A PDF boolean
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub struct PdfBoolean(bool);

impl PdfBoolean {
    /// Create a new boolean
    pub fn new(val: bool) -> PdfBoolean {
        PdfBoolean(val)
    }
}

impl_into_from_case! { PdfObject : PdfObject::Boolean => PdfBoolean }

impl Output for PdfBoolean {
    fn output(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::PdfBoolean;
    use output::Output;

    #[test]
    fn output_true() {
        const EXPECTED: &'static str = "true";
        let actual = PdfBoolean::new(true).output();
        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn output_false() {
        const EXPECTED: &'static str = "false";
        let actual = PdfBoolean::new(false).output();
        assert_eq!(actual, EXPECTED);
    }
}
