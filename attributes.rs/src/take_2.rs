// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub trait Attribute {
    /// Implementations of `Attribute` will specify the type of the value
    /// that they work with.
    type ValueType;

    /// Given the type, get back the actual instance of this
    /// object.
    fn type_to_value() -> &'static Self;
}

/// Link an object and an attribute.
pub trait EntityAttribute<A: Attribute> {
    fn get_attr(&self, attr: &A) -> A::ValueType;
}

/// This lets us access a value via the type object for
/// direct dispatch.
pub trait Attr<A: 'static + Attribute> {
    fn get(&self) -> A::ValueType;
}

/// Generic impl for anything where EntityAttribute is already
/// implemented.
impl<A, E> Attr<A> for E
    where A: 'static + Attribute,
          E: EntityAttribute<A>
{
    fn get(&self) -> A::ValueType {
        let attr = A::type_to_value();
        self.get_attr(attr)
    }
}

/// Duration is just an empty struct, a marker type / object.
pub struct Duration {}

/// Set up the value level object that mirrors the type level value.
pub static DURATION: Duration = Duration {};

/// Establish the link between type and value levels for Duration
/// and that it is an `i32`.
impl Attribute for Duration {
    type ValueType = i32;

    fn type_to_value() -> &'static Self {
        &DURATION
    }
}

/// StartTime is just an empty struct, a marker type / object.
pub struct StartTime {}

pub static START_TIME: StartTime = StartTime {};

/// Establish the link between type and value levels for StartTime
/// and that it is an `i32`.
impl Attribute for StartTime {
    type ValueType = i32;
    fn type_to_value() -> &'static Self {
        &START_TIME
    }
}

/// A sample object ... this has a Duration and a StartTime.
pub struct Event {
    pub duration: i32,
    pub start_time: i32,
}

/// Return the `duration` field for the `Duration` attribute.
impl EntityAttribute<Duration> for Event {
    fn get_attr(&self, _attr: &Duration) -> i32 {
        self.duration
    }
}

/// Return the `start_time` field for the `StartTime` attribute.
impl EntityAttribute<StartTime> for Event {
    fn get_attr(&self, _attr: &StartTime) -> i32 {
        self.start_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let e1 = Event {
            duration: 3,
            start_time: 5,
        };
        let e2 = Event {
            duration: 298,
            start_time: 89,
        };

        assert_eq!(e1.get_attr(&DURATION), 3);
        assert_eq!(e1.get_attr(&START_TIME), 5);
        assert_eq!(e2.get_attr(&DURATION), 298);
        assert_eq!(e2.get_attr(&START_TIME), 89);

        assert_eq!(Attr::<StartTime>::get(&e1), 5);
        assert_eq!(Attr::<StartTime>::get(&e2), 89);

        // But we can't do this ...
        // let a: &Attribute = if (3) { &Duration } else { &StartTime };
        // assert_eq!(e1.get_attr(a), 3);
        // That's because we need compile time awareness of the types
        // of the attribute and the value.
    }
}
