use objects::PdfArray;
use objects::PdfInteger;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Rectangle(pub i64, pub i64, pub i64, pub i64);

impl Rectangle {
    pub fn to_array(&self) -> PdfArray {
        let vec: Vec<PdfInteger> =
            [self.0, self.1, self.2, self.3]
                .into_iter()
                .map(|v| PdfInteger::new(*v))
                .collect();
        PdfArray::from(vec)
    }
}