use super::PdfObject;

use output::Output;

const MAX_ENTRIES_FOR_SINGLE_LINE_OUTPUT: usize = 4;

/// A PDF array
#[derive(Debug)]
pub struct PdfArray {
    repr: Vec<PdfObject>
}

impl PdfArray {
    /// Create a new array
    pub fn new() -> PdfArray {
        PdfArray { repr: Vec::new() }
    }

    /// Create a new array from a vector of PdfObjects
    pub fn from<T>(vec: Vec<T>) -> PdfArray
            where T: Into<PdfObject> {
        PdfArray { repr: vec.into_iter().map(|t| t.into()).collect() }
    }
}

impl_into_from_case! { PdfObject : PdfObject::Array => PdfArray }

impl Output for PdfArray {
    fn output(&self) -> String {
        let mut builder = String::new();
        let seperator =
            if self.repr.len() > MAX_ENTRIES_FOR_SINGLE_LINE_OUTPUT { '\n' }
            else { ' ' };
        builder.push('[');
        for kid in &self.repr {
            builder.push(seperator);
            builder.push_str(&kid.output());
        }
        builder.push(seperator);
        builder.push(']');
        builder
    }
}

#[cfg(test)]
mod test {
    use super::MAX_ENTRIES_FOR_SINGLE_LINE_OUTPUT;
    use super::PdfArray;
    use output::Output;

    /// Test an empty array
    #[test]
    fn output_empty() {
        const EXPECTED: &'static str = "[ ]";
        let actual = PdfArray::new().output();
        assert_eq!(actual, EXPECTED);
    }

    /// Use just PdfArrays instead of other children.
    /// This way this test is sanitized against changes in other objects.
    #[test]
    fn output_one_line() {
        const EXPECTED: &'static str = "[ [ ] [ ] [ ] [ ] ]";
        let mut array = PdfArray::new();
        for _ in 0 .. MAX_ENTRIES_FOR_SINGLE_LINE_OUTPUT {
            // HACK: This uses the repr. Shouldn't.
            array.repr.push(PdfArray::new().into());
        }
        let actual = array.output();
        assert_eq!(actual, EXPECTED);
    }

    /// Use just PdfArrays instead of other children.
    /// This way this test is sanitized against changes in other objects.
    #[test]
    fn output_multiple_lines() {
        const EXPECTED: &'static str = "[\n[ ]\n[ ]\n[ ]\n[ ]\n[ ]\n]";
        let mut array = PdfArray::new();
        for _ in 0 .. MAX_ENTRIES_FOR_SINGLE_LINE_OUTPUT + 1 {
            // HACK: This uses the repr. Shouldn't.
            array.repr.push(PdfArray::new().into());
        }
        let actual = array.output();
        assert_eq!(actual, EXPECTED);
    }
}
