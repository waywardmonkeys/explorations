// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// `Attribute` here combines storage and type.
///
/// This is actually a bad idea because we want to separate
/// the storage of the value from the concept of its type.
pub trait Attribute {
    /// Implementations of `Attribute` will specify the type of the value
    /// that they work with.
    type ValueType;

    /// Given the type, get back the actual instance of this
    /// object.
    fn type_to_value() -> &'static Self;

    /// Return the value of this attribute object.
    fn value(&self) -> Self::ValueType;
}

/// A `Duration` stores an `i32` value.
pub struct Duration {
    pub v: i32,
}

/// And here, we impl `Attribute` for `Duration`.
impl Attribute for Duration {
    type ValueType = i32;

    fn type_to_value() -> &'static Self {
        &Duration
    }

    fn value(&self) -> Self::ValueType {
        self.v
    }
}

#[allow(non_upper_case_globals)]
pub static Duration: Duration = Duration { v: 0 };

/// This lets us get the value of an attribute, using the value form of
/// the attribute.
pub fn get_attr<A>(attr: &A) -> Option<A::ValueType>
    where A: Attribute
{
    Some(attr.value())
}

/// This lets us use the compile time type to, hopefully, get right to
/// the correct code. Does this happen in an optimal way with inlining?
pub fn get_attr_value<A: 'static + Attribute>() -> Option<A::ValueType> {
    let attr = A::type_to_value();
    Some(attr.value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(get_attr(&Duration), Some(0));
        assert_eq!(get_attr_value::<Duration>(), Some(0));
    }
}
