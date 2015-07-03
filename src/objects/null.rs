use super::PdfObject;

use output::Output;

/// A PDF null
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct PdfNull;

impl_into_from_case! { PdfObject : PdfObject::Null => PdfNull }

impl Output for PdfNull {
    fn output(&self) -> String {
        "null".to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::PdfNull;
    use output::Output;

    #[test]
    fn output() {
        const EXPECTED: &'static str = "null";
        let actual = PdfNull.output();
        assert_eq!(actual, EXPECTED);
    }
}
