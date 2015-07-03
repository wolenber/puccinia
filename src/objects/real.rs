use super::PdfObject;

use output::Output;

/// A PDF real
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub struct PdfReal(f64);

impl PdfReal {
    /// Create a new PDF real
    pub fn new(val: f64) -> PdfReal {
        PdfReal(val)
    }
}

impl_simple_arithmetic! { PdfReal }

impl_into_from_case! { PdfObject : PdfObject::Real => PdfReal }

impl Output for PdfReal {
    fn output(&self) -> String {
        // HACK: There's probably a better way to guarantee at least once decimal.
        let min_dec = format!("{:.1}", self.0);
        let dec = format!("{}", self.0);
        if min_dec.len() > dec.len() {
            min_dec
        } else {
            dec
        }
    }
}

#[cfg(test)]
mod test {
    use super::PdfReal;
    use output::Output;

    #[test]
    fn output_seven() {
        const EXPECTED: &'static str = "7.5";
        let actual = PdfReal::new(7.5).output();
        assert_eq!(actual, EXPECTED);
    }
}
