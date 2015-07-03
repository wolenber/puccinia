use super::PdfObject;
use output::Output;

/// A PDF reference
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct PdfReference {
    /// Identity number of the reference
    pub id: u32,
    /// Generation of the reference
    pub gen: u16
}

impl PdfReference {
    /// Create a new PDF reference
    pub fn new(id: u32) -> PdfReference {
        PdfReference {
            id: id,
            gen: 0
        }
    }

    pub fn next(&self) -> PdfReference {
        PdfReference::new(self.id + 1)
    }
}

impl_into_from_case! { PdfObject : PdfObject::Reference => PdfReference }

impl Output for PdfReference {
    fn output(&self) -> String {
        format!("{} {} R", self.id, self.gen)
    }
}

#[cfg(test)]
mod test {
    use super::PdfReference;
    use output::Output;

    #[test]
    fn output() {
        const EXPECTED: &'static str = "1 0 R";
        let actual = PdfReference::new(1).output();
        assert_eq!(actual, EXPECTED);
    }
}
