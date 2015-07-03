/// Output allows easier writing to a pdf
pub trait Output {
    /// Create a string which properly represents self.
    fn output(&self) -> String;
}
