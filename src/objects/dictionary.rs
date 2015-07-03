use super::PdfObject;

use output::Output;

use std::borrow::Borrow;
use std::collections::BTreeMap;

/// A PDF dictionary
#[derive(Debug)]
pub struct PdfDictionary {
    repr: BTreeMap<String, PdfObject>
}

impl PdfDictionary {
    /// Create a new dictionary
    pub fn new() -> PdfDictionary {
        PdfDictionary{ repr: BTreeMap::new() }
    }

    /// Return true if this dictionary contains a key. False otherwise.
    pub fn contains<K: ?Sized>(&self, key: &K) -> bool
            where K: Ord, String: Borrow<K> {
        self.repr.contains_key(key)
    }

    /// Get a V out of the dictionary.
    /// Returns None if either:
    ///     the key doesn't exist;
    ///     or the object is not of that type.
    pub fn get<'a, K: ?Sized, V>(&'a self, key: &K) -> Option<&V>
            where K: Ord, String: Borrow<K>,
                  &'a PdfObject: Into<Option<&'a V>> {
        self.repr.get(key).and_then(|obj| obj.into())
    }

    /// Get a mutable V out of the dictionary.
    /// Returns None if either:
    ///     the key doesn't exist;
    ///     or the object is not of that type.
    pub fn get_mut<'a, K: ?Sized, V>(&'a mut self, key: &K) -> Option<&'a mut V>
            where K: Ord, String: Borrow<K>,
                  &'a mut PdfObject: Into<Option<&'a mut V>> {
        self.repr.get_mut(key).and_then(|obj| obj.into())
    }

    /// Set a value of the dictionary.
    pub fn set<'a, K: ?Sized, V>(&mut self, key: &'a K, value: V)
            where &'a K: Into<String>,
                  V: Into<PdfObject> {
        self.repr.insert(key.into(), value.into());
    }
}

impl_into_from_case! { PdfObject : PdfObject::Dictionary => PdfDictionary }

impl Output for PdfDictionary {
    fn output(&self) -> String {
        let mut builder = String::new();
        builder.push_str("<<\n");
        for (key, val) in self.repr.iter() {
            builder.push_str(&format!("/{} {}", key, val.output()));
            builder.push('\n');
        }
        builder.push_str(">>");
        builder
    }
}

#[cfg(test)]
mod test {
    use super::PdfDictionary;
    use output::Output;

    #[test]
    fn output_empty() {
        const EXPECTED: &'static str = "<<\n>>";
        let actual = PdfDictionary::new().output();
        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn output_populated() {
        const EXPECTED: &'static str = "<<\n/Foo <<\n>>\n>>";
        let mut dict = PdfDictionary::new();
        dict.set("Foo", PdfDictionary::new());
        let actual = dict.output();
        assert_eq!(actual, EXPECTED);
    }
}
