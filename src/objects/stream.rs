use super::PdfDictionary;
use super::PdfInteger;
use super::PdfObject;

use output::Output;

/// A PDF stream
#[derive(Debug)]
#[derive(Clone)]
pub struct PdfStream {
    contents: Vec<String>,
    length: usize
}

impl PdfStream {
    /// Create a new stream
    pub fn new() -> PdfStream {
        PdfStream {
            contents: Vec::new(),
            length: 0
        }
    }

    /// Append a line to the stream
    pub fn append_line(&mut self, s: &str) -> &mut Self {
        // Remember the +1 for newline
        self.length += s.len() + 1;
        self.contents.push(s.to_owned());
        self
    }
}

impl_into_from_case! { PdfObject : PdfObject::Stream => PdfStream }

impl Output for PdfStream {
    fn output(&self) -> String {
        let mut dict = PdfDictionary::new();
        dict.set("Length", PdfInteger::new(self.length as i64));
        let dict = dict;

        let contents = self.contents.iter()
            .fold("".to_owned(), |acc, item|
                if acc == "" {
                    acc + item
                } else {
                    acc + "\n" + item
                });

        format!("{}\nstream\n{}\nendstream", dict.output(), contents)
    }
}


#[cfg(test)]
mod test {
    use super::PdfStream;
    use output::Output;

    #[test]
    fn output_empty() {
        const EXPECTED: &'static str = "<<\n/Length 1\n>>\nstream\nendstream";
        let actual = PdfStream::new().output();
        assert_eq!(actual, EXPECTED);
    }
}
