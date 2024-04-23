mod impls;

use core::fmt;
mod error;

pub use error::JsonError;

pub mod prelude {
    pub use crate::{error::JsonError, Json, JsonArray, JsonNumber, JsonObject};
}

pub trait Json: fmt::Debug {
    type Object: JsonObject<Value = Self>;
    type Array: JsonArray<Element = Self>;
    type String: PartialEq<str> + Eq + fmt::Debug + AsRef<str> + ?Sized;
    type Number: for<'a> JsonNumber<'a>;

    fn as_object(&self) -> Option<&Self::Object> {
        self.try_as_object()
            .expect("Failed to convert value to Object")
    }
    fn as_array(&self) -> Option<&Self::Array> {
        self.try_as_array()
            .expect("Failed to convert value to Array")
    }
    fn as_string(&self) -> Option<&Self::String> {
        self.try_as_string()
            .expect("Failed to convert value to String")
    }
    fn as_number(&self) -> Option<&Self::Number> {
        self.try_as_number()
            .expect("Failed to convert value to Number")
    }
    fn as_boolean(&self) -> Option<bool> {
        self.try_as_boolean()
            .expect("Failed to convert value to Boolean")
    }
    fn as_null(&self) -> Option<()> {
        self.try_as_null().expect("Failed to convert value to Null")
    }
    fn try_as_object(&self) -> Result<Option<&Self::Object>, JsonError>;
    fn try_as_array(&self) -> Result<Option<&Self::Array>, JsonError>;
    fn try_as_string(&self) -> Result<Option<&Self::String>, JsonError>;
    fn try_as_number(&self) -> Result<Option<&Self::Number>, JsonError>;
    fn try_as_boolean(&self) -> Result<Option<bool>, JsonError>;
    fn try_as_null(&self) -> Result<Option<()>, JsonError>;
    fn is_object(&self) -> bool {
        self.as_object().is_some()
    }
    fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
    fn is_string(&self) -> bool {
        self.as_string().is_some()
    }
    fn is_number(&self) -> bool {
        self.as_number().is_some()
    }
    fn is_boolean(&self) -> bool {
        self.as_boolean().is_some()
    }
    fn is_null(&self) -> bool {
        self.as_null().is_some()
    }
    fn try_is_object(&self) -> Result<bool, JsonError> {
        self.try_as_object().map(|opt| opt.is_some())
    }
    fn try_is_array(&self) -> Result<bool, JsonError> {
        self.try_as_array().map(|opt| opt.is_some())
    }
    fn try_is_string(&self) -> Result<bool, JsonError> {
        self.try_as_string().map(|opt| opt.is_some())
    }
    fn try_is_number(&self) -> Result<bool, JsonError> {
        self.try_as_number().map(|opt| opt.is_some())
    }
    fn try_is_boolean(&self) -> Result<bool, JsonError> {
        self.try_as_boolean().map(|opt| opt.is_some())
    }
    fn try_is_null(&self) -> Result<bool, JsonError> {
        self.try_as_null().map(|opt| opt.is_some())
    }
    fn equal(&self, other: &Self) -> bool {
        self.try_equal(other).expect("Failed to compare values")
    }
    fn try_equal(&self, other: &Self) -> Result<bool, JsonError>;
}

pub trait JsonObject {
    type Key: AsRef<str> + ?Sized;
    type Value: Json;
    type Iter<'a>: Iterator<Item = (Result<&'a Self::Key, JsonError>, &'a Self::Value)>
    where
        Self: 'a;

    fn get(&self, key: &str) -> Option<&Self::Value>;

    fn iter(&self) -> Self::Iter<'_>;

    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

pub trait JsonArray {
    type Element: Json;
    type Iter<'a>: Iterator<Item = Result<&'a Self::Element, JsonError>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.try_iter().expect("Failed to iterate")
    }
    fn try_iter(&self) -> Result<Self::Iter<'_>, JsonError>;

    fn get(&self, idx: usize) -> Option<&Self::Element> {
        self.try_get(idx)
            .unwrap_or_else(|err| panic!("Failed to get an element at index {idx}: {err}"))
    }
    fn try_get(&self, idx: usize) -> Result<Option<&Self::Element>, JsonError>;
}

pub trait JsonNumber<'a> {
    type Integer: fmt::Debug + 'a;

    fn as_integer<I: TryFrom<Self::Integer>>(&'a self) -> Option<Self::Integer>;
    fn try_into_integer<I: TryFrom<Self::Integer, Error = E>, E>(&'a self) -> Result<I, E> {
        self.as_integer::<I>()
            .expect("Should be an integer")
            .try_into()
    }
    fn as_float(&self) -> Option<f64>;
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, panic};

    use crate::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct CustomInteger {
        pub(crate) v: i64,
    }

    impl CustomInteger {
        pub(crate) fn new(v: i64) -> Self {
            Self { v }
        }
    }

    impl From<i64> for CustomInteger {
        fn from(value: i64) -> Self {
            Self::new(value)
        }
    }

    pub(crate) fn assert_object_get(value: &impl Json) {
        let object = value.as_object().expect("Should be an object");
        assert!(object.get("a").is_some());
        assert!(object.get("b").is_none());
        let val = value
            .as_object()
            .and_then(|obj| obj.get("c"))
            .and_then(|k| k.as_string())
            .map(|s| s.borrow().to_owned())
            .expect("Key exists");
        assert_eq!(val, "d");
    }

    pub(crate) fn assert_object_iter<'a, J>(value: &'a J)
    where
        J: Json,
        CustomInteger: TryFrom<
            <<<J::Object as JsonObject>::Value as Json>::Number as JsonNumber<'a>>::Integer,
        >,
    {
        let object = value.as_object().expect("Should be an object");
        let mut iter = object.iter();
        let (key, value) = iter.next().expect("Empty object");
        let Ok(key) = key else {
            panic!("Invalid key");
        };
        assert_eq!(key.as_ref(), "a");
        let Ok(integer) = value
            .as_number()
            .expect("Should be a number")
            .try_into_integer::<CustomInteger, _>()
        else {
            panic!("Failed to convert to integer")
        };
        assert_eq!(integer, CustomInteger::new(1));
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    pub(crate) fn assert_array_iter<'a, J>(value: &'a J)
    where
        J: Json,
        CustomInteger: TryFrom<
            <<<J::Array as JsonArray>::Element as Json>::Number as JsonNumber<'a>>::Integer,
        >,
    {
        let array = value.as_array().expect("Should be an array");
        let element = array
            .iter()
            .next()
            .expect("Empty array")
            .expect("Failed to iterate");
        let Ok(integer) = element
            .as_number()
            .expect("Should be a number")
            .try_into_integer::<CustomInteger, _>()
        else {
            panic!("Failed to convert to integer")
        };
        assert_eq!(integer, CustomInteger::new(1));
    }

    pub(crate) fn assert_array_get(value: &impl Json) {
        let array = value.as_array().expect("Should be an array");
        assert!(array.get(0).is_some());
        assert!(array.get(1).is_none());
    }

    pub(crate) fn assert_as_object(value: &impl Json, expected: bool) {
        assert_eq!(value.as_object().is_some(), expected);
        if expected {
            assert!(value.is_object());
            assert!(value.try_is_object().expect("Failed to call is_object"));
        }
    }

    pub(crate) fn assert_as_array(value: &impl Json, expected: bool) {
        assert_eq!(value.as_array().is_some(), expected);
        if expected {
            assert!(value.is_array());
            assert!(value.try_is_array().expect("Failed to call is_array"));
        }
    }

    pub(crate) fn assert_as_string(value: &impl Json, expected: Option<&str>) {
        match (value.as_string(), expected) {
            (Some(s), Some(e)) => assert_eq!(s.borrow(), e),
            (None, None) => {}
            _ => panic!("Value is not {:?}", expected),
        }
        if expected.is_some() {
            assert!(value.is_string());
            assert!(value.try_is_string().expect("Failed to call is_string"));
        }
    }

    pub(crate) fn assert_as_number_integer<'a, J, I>(value: &'a J, expected: Option<I>)
    where
        J: Json,
        I: TryFrom<<J::Number as JsonNumber<'a>>::Integer> + PartialEq<I>,
        <<J as Json>::Number as JsonNumber<'a>>::Integer: core::fmt::Debug,
    {
        let number = value.as_number();
        if let Some(expected) = expected {
            let number = number.expect("Not a number");
            let Ok(integer) = number.try_into_integer::<I, _>() else {
                panic!("Failed to convert to integer")
            };
            if integer != expected {
                panic!("Values are not equal")
            }
            assert!(value.is_number());
            assert!(value.try_is_number().expect("Failed to call is_number"));
        } else if let Some(n) = number {
            if let Some(int) = n.as_integer::<I>() {
                assert!(TryInto::<I>::try_into(int).is_err());
            }
        }
    }

    pub(crate) fn assert_as_number_float(value: &impl Json, expected: Option<f64>) {
        let number = value.as_number();
        if let Some(expected) = expected {
            let number = number.expect("Not a number");
            let integer = number.as_float().expect("Should be a float");
            if integer != expected {
                panic!("Values are not equal")
            }
            assert!(value.is_number());
            assert!(value.try_is_number().expect("Failed to call is_number"));
        } else {
            assert!(number.is_none());
        }
    }

    pub(crate) fn assert_as_boolean(value: &impl Json, expected: Option<bool>) {
        match (value.as_boolean(), expected) {
            (Some(s), Some(e)) => assert_eq!(*s.borrow(), e),
            (None, None) => {}
            _ => panic!("Value is not {:?}", expected),
        }
        if expected.is_some() {
            assert!(value.is_boolean());
            assert!(value.try_is_boolean().expect("Failed to call is_boolean"));
        }
    }

    pub(crate) fn assert_as_null(value: &impl Json, expected: Option<()>) {
        assert_eq!(value.as_null(), expected);
        if expected.is_some() {
            assert!(value.is_null());
            assert!(value.try_is_null().expect("Failed to call is_null"));
        }
    }
}
