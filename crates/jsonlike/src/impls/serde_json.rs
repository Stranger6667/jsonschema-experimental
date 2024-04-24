use crate::prelude::*;

pub struct ObjectIter<'a>(serde_json::map::Iter<'a>);

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (Result<&'a String, JsonError>, &'a serde_json::Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (Ok(k), v))
    }
}

impl JsonObject for serde_json::Map<String, serde_json::Value> {
    type Key = String;
    type Value = serde_json::Value;
    type Iter<'a> = ObjectIter<'a>;

    fn get(&self, key: &str) -> Option<&Self::Value> {
        <serde_json::Map<String, serde_json::Value>>::get(self, key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        ObjectIter(<serde_json::Map<String, serde_json::Value>>::iter(self))
    }
}

pub struct ArrayIter<'a>(std::slice::Iter<'a, serde_json::Value>);

impl<'a> Iterator for ArrayIter<'a> {
    type Item = Result<&'a serde_json::Value, JsonError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Ok)
    }
}

impl JsonArray for Vec<serde_json::Value> {
    type Element = serde_json::Value;
    type Iter<'a> = ArrayIter<'a>;

    fn try_iter(&self) -> Result<Self::Iter<'_>, JsonError> {
        Ok(ArrayIter(<[serde_json::Value]>::iter(self)))
    }

    fn try_get(&self, idx: usize) -> Result<Option<&Self::Element>, JsonError> {
        Ok(<[serde_json::Value]>::get(self, idx))
    }
}

#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Debug, Copy, Clone)]
pub enum SerdeInteger {
    PosInt(u64),
    NegInt(i64),
}

impl<'a> JsonNumber<'a> for serde_json::Number {
    #[cfg(feature = "arbitrary_precision")]
    type Integer = &'a str;
    #[cfg(not(feature = "arbitrary_precision"))]
    type Integer = SerdeInteger;

    fn as_integer<I: TryFrom<Self::Integer>>(&'a self) -> Option<Self::Integer> {
        #[cfg(feature = "arbitrary_precision")]
        {
            Some(self.as_str())
        }
        #[cfg(not(feature = "arbitrary_precision"))]
        {
            if let Some(int) = self.as_u64() {
                Some(SerdeInteger::PosInt(int))
            } else {
                self.as_i64().map(SerdeInteger::NegInt)
            }
        }
    }
    fn as_float(&self) -> Option<f64> {
        self.as_f64()
    }
}

impl Json for serde_json::Value {
    type Object = serde_json::Map<String, serde_json::Value>;
    type Array = Vec<serde_json::Value>;
    type String = str;
    type Number = serde_json::Number;

    fn try_as_object(
        &self,
    ) -> Result<Option<&serde_json::Map<String, serde_json::Value>>, JsonError> {
        Ok(self.as_object())
    }

    fn try_as_array(&self) -> Result<Option<&Vec<serde_json::Value>>, JsonError> {
        Ok(self.as_array())
    }

    fn try_as_string(&self) -> Result<Option<&str>, JsonError> {
        Ok(self.as_str())
    }

    fn try_as_number(&self) -> Result<Option<&serde_json::Number>, JsonError> {
        Ok(self.as_number())
    }

    fn try_as_boolean(&self) -> Result<Option<bool>, JsonError> {
        Ok(self.as_bool())
    }

    fn try_as_null(&self) -> Result<Option<()>, JsonError> {
        Ok(self.as_null())
    }

    fn try_equal(&self, other: &Self) -> Result<bool, JsonError> {
        Ok(self.eq(other))
    }
    fn from_str(s: &str) -> Result<Self, JsonError>
    where
        Self: Sized,
    {
        serde_json::from_str(s).map_err(|err| JsonError::new(Box::new(err)))
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use serde_json::{json, Value};
    use test_case::test_case;

    use crate::{
        tests::{
            assert_array_get, assert_array_iter, assert_as_array, assert_as_boolean,
            assert_as_null, assert_as_number_float, assert_as_number_integer, assert_as_object,
            assert_as_string, assert_object_get, assert_object_iter, CustomInteger,
        },
        Json,
    };

    #[cfg(not(feature = "arbitrary_precision"))]
    use super::SerdeInteger;

    #[cfg(feature = "arbitrary_precision")]
    impl TryFrom<&str> for CustomInteger {
        type Error = ParseIntError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(CustomInteger::new(value.parse()?))
        }
    }

    #[cfg(not(feature = "arbitrary_precision"))]
    impl TryFrom<SerdeInteger> for CustomInteger {
        type Error = ParseIntError;

        fn try_from(value: SerdeInteger) -> Result<Self, Self::Error> {
            match value {
                SerdeInteger::PosInt(i) => Ok(CustomInteger::new(i as i64)),
                SerdeInteger::NegInt(i) => Ok(CustomInteger::new(i)),
            }
        }
    }

    fn build_object() -> Value {
        json!({"a": 1, "c": "d"})
    }

    fn build_array() -> Value {
        json!([1])
    }

    #[test]
    fn test_object_get() {
        assert_object_get(&build_object());
    }

    #[test]
    fn test_object_iter() {
        assert_object_iter(&build_object());
    }

    #[test]
    fn test_array_iter() {
        assert_array_iter(&build_array());
    }

    #[test]
    fn test_array_get() {
        assert_array_get(&build_array());
    }

    #[test_case(json!(null), None)]
    #[test_case(json!(true), None)]
    #[test_case(json!(42), Some(CustomInteger::new(42)))]
    #[test_case(json!(5.15), None)]
    #[test_case(json!("abc"), None)]
    #[test_case(json!([1]), None)]
    #[test_case(json!({"a": 1}), None)]
    fn test_as_number_integer(value: Value, expected: Option<CustomInteger>) {
        assert_as_number_integer(&value, expected);
    }

    #[test_case(json!(null), None)]
    #[test_case(json!(true), None)]
    #[test_case(json!(42), Some(42.0))]
    #[test_case(json!(5.15), Some(5.15))]
    #[test_case(json!("abc"), None)]
    #[test_case(json!([1]), None)]
    #[test_case(json!({"a": 1}), None)]
    fn test_as_number_float(value: Value, expected: Option<f64>) {
        assert_as_number_float(&value, expected);
    }

    #[test_case(json!(null), false)]
    #[test_case(json!(true), false)]
    #[test_case(json!(5.15), false)]
    #[test_case(json!(42), false)]
    #[test_case(json!("abc"), false)]
    #[test_case(json!([1]), false)]
    #[test_case(json!({"a": 1}), true)]
    fn test_as_object(value: Value, expected: bool) {
        assert_as_object(&value, expected);
    }

    #[test_case(json!(null), false)]
    #[test_case(json!(true), false)]
    #[test_case(json!(5.15), false)]
    #[test_case(json!(42), false)]
    #[test_case(json!("abc"), false)]
    #[test_case(json!([1]), true)]
    #[test_case(json!({"a": 1}), false)]
    fn test_as_array(value: Value, expected: bool) {
        assert_as_array(&value, expected);
    }

    #[test_case(json!(null), None)]
    #[test_case(json!(true), None)]
    #[test_case(json!(5.15), None)]
    #[test_case(json!(42), None)]
    #[test_case(json!("abc"), Some("abc"))]
    #[test_case(json!([1]), None)]
    #[test_case(json!({"a": 1}), None)]
    fn test_as_string(value: Value, expected: Option<&str>) {
        assert_as_string(&value, expected);
    }

    #[test_case(json!(null), None)]
    #[test_case(json!(true), Some(true))]
    #[test_case(json!(5.15), None)]
    #[test_case(json!(42), None)]
    #[test_case(json!("abc"), None)]
    #[test_case(json!([1]), None)]
    #[test_case(json!({"a": 1}), None)]
    fn test_as_boolean(value: Value, expected: Option<bool>) {
        assert_as_boolean(&value, expected);
    }

    #[test_case(json!(null), Some(()))]
    #[test_case(json!(true), None)]
    #[test_case(json!(5.15), None)]
    #[test_case(json!(42), None)]
    #[test_case(json!("abc"), None)]
    #[test_case(json!([1]), None)]
    #[test_case(json!({"a": 1}), None)]
    fn test_as_null(value: Value, expected: Option<()>) {
        assert_as_null(&value, expected);
    }

    #[test_case(json!(null))]
    #[test_case(json!(true))]
    #[test_case(json!(5.15))]
    #[test_case(json!(42))]
    #[test_case(json!("abc"))]
    #[test_case(json!([1]))]
    #[test_case(json!({"a": 1}))]
    fn test_equal(value: Value) {
        assert!(value.equal(&value));
        assert!(!value.equal(&json!("something else")));
    }

    #[test]
    fn test_from_str() {
        let value = json!({"a": 1});
        let s = value.to_string();
        let parsed = Value::from_str(&s).unwrap();
        assert_eq!(value, parsed);
    }
}
