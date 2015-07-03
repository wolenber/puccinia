//! Module containing the various basic types of a pdf object

use output::Output;

#[macro_use]
mod macros;

mod boolean;
mod integer;
mod real;
mod string;
mod name;
mod array;
mod dictionary;
mod stream;
mod null;
mod reference;

pub use self::boolean::PdfBoolean;
pub use self::integer::PdfInteger;
pub use self::real::PdfReal;
pub use self::string::PdfString;
pub use self::name::PdfName;
pub use self::array::PdfArray;
pub use self::dictionary::PdfDictionary;
pub use self::stream::PdfStream;
pub use self::null::PdfNull;
pub use self::reference::PdfReference;

/// A PDF object of any type
#[derive(Debug)]
pub enum PdfObject {
    /// A PDF boolean
    Boolean(PdfBoolean),
    /// A PDF integer
    Integer(PdfInteger),
    /// A PDF real
    Real(PdfReal),
    /// A PDF string
    String(PdfString),
    /// A PDF name
    Name(PdfName),
    /// A PDF array
    Array(PdfArray),
    /// A PDF dictionary
    Dictionary(PdfDictionary),
    /// A PDF stream
    Stream(PdfStream),
    /// A PDF null
    Null(PdfNull),
    /// A PDF reference
    Reference(PdfReference),
}

impl Output for PdfObject {
    fn output(&self) -> String {
        use self::PdfObject::*;
        match *self {
            Boolean(ref val) => val.output(),
            Integer(ref val) => val.output(),
            Real(ref val) => val.output(),
            String(ref val) => val.output(),
            Name(ref val) => val.output(),
            Array(ref val) => val.output(),
            Dictionary(ref val) => val.output(),
            Stream(ref val) => val.output(),
            Null(ref val) => val.output(),
            Reference(ref val) => val.output(),
        }
    }
}
