use super::PdfObject;

use output::Output;

/// A PDF integer
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct PdfInteger(i64);

impl PdfInteger {
    /// Create a new PDF integer
    pub fn new(val: i64) -> PdfInteger {
        PdfInteger(val)
    }

    /// Increment the integer
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl_simple_arithmetic! { PdfInteger }

impl_into_from_case! { PdfObject : PdfObject::Integer => PdfInteger }

impl Output for PdfInteger {
    fn output(&self) -> String {
        self.0.to_string()
    }
}


#[cfg(test)]
mod test {
    use super::PdfInteger;
    use output::Output;

    #[test]
    fn output_seven() {
        const EXPECTED: &'static str = "7";
        let actual = PdfInteger::new(7).output();
        assert_eq!(actual, EXPECTED);
    }
}
