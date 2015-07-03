// Cover the From/Into impls for types a lot simpler
macro_rules! impl_into_from_case {
    ( $broad:ty : $case:path => $specific:ty ) => {
        impl From<$specific> for $broad {
            fn from(val: $specific) -> $broad {
                $case(val)
            }
        }

        impl <'a> From<&'a $broad> for Option<&'a $specific> {
            fn from(val: &'a $broad) -> Option<&'a $specific> {
                match val {
                    &$case(ref val) => Some(val),
                    _ => None
                }
            }
        }

        impl <'a> From<&'a mut $broad> for Option<&'a mut $specific> {
            fn from(val: &'a mut $broad) -> Option<&'a mut $specific> {
                match val {
                    &mut $case(ref mut val) => Some(val),
                    _ => None
                }
            }
        }
    }
}

// Simple macro to save some duplication across Integer/Real
macro_rules! impl_simple_arithmetic {
    ( $kind:ident ) => {
        impl ::std::ops::Add for $kind {
            type Output = $kind;
            fn add(self, other: $kind) -> $kind {
                $kind(self.0 + other.0)
            }
        }
        impl ::std::ops::Sub for $kind {
            type Output = $kind;
            fn sub(self, other: $kind) -> $kind {
                $kind(self.0 - other.0)
            }
        }
        impl ::std::ops::Mul for $kind {
            type Output = $kind;
            fn mul(self, other: $kind) -> $kind {
                $kind(self.0 * other.0)
            }
        }
        impl ::std::ops::Div for $kind {
            type Output = $kind;
            fn div(self, other: $kind) -> $kind {
                $kind(self.0 / other.0)
            }
        }
    }
}
