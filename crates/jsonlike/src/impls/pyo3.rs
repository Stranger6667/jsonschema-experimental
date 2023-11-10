use crate::prelude::*;
use pyo3::{
    exceptions::PyIndexError,
    prelude::*,
    types::{
        iter::PyDictIterator, PyBool, PyDict, PyFloat, PyIterator, PyList, PyLong, PyNone, PyString,
    },
};

pub struct PyDictIteratorAdapter<'a>(PyDictIterator<'a>);

impl From<PyErr> for JsonError {
    fn from(value: PyErr) -> Self {
        JsonError::new(Box::new(value))
    }
}

impl<'a> Iterator for PyDictIteratorAdapter<'a> {
    type Item = (Result<&'a str, JsonError>, &'a PyAny);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, value)) = self.0.next() {
            match key.downcast::<PyString>() {
                Ok(key) => match key.to_str() {
                    Ok(key) => Some((Ok(key), value)),
                    Err(error) => Some((Err(error.into()), value)),
                },
                Err(error) => Some((Err(JsonError::from(PyErr::from(error))), value)),
            }
        } else {
            None
        }
    }
}

impl JsonObject for PyDict {
    type Key = str;
    type Value = PyAny;
    type Iter<'a> = PyDictIteratorAdapter<'a>;

    fn get(&self, key: &str) -> Option<&Self::Value> {
        // SAFETY: This should never panic as key is hashable (&str)
        self.get_item(key).expect("Invalid key")
    }

    fn iter(&self) -> Self::Iter<'_> {
        PyDictIteratorAdapter(PyDict::iter(self))
    }
}

pub struct PyIteratorAdapter<'a>(&'a PyIterator);

impl<'a> Iterator for PyIteratorAdapter<'a> {
    type Item = Result<&'a PyAny, JsonError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| i.map_err(JsonError::from))
    }
}

impl JsonArray for PyList {
    type Element = PyAny;
    type Iter<'a> = PyIteratorAdapter<'a>;

    fn try_iter(&self) -> Result<Self::Iter<'_>, JsonError> {
        Ok(PyIteratorAdapter(PyIterator::from_object(self)?))
    }

    fn try_get(&self, idx: usize) -> Result<Option<&Self::Element>, JsonError> {
        match self.get_item(idx) {
            Ok(item) => Ok(Some(item)),
            Err(err) if err.is_instance_of::<PyIndexError>(self.py()) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

impl<'a> JsonNumber<'a> for PyAny {
    #[cfg(feature = "arbitrary_precision")]
    type Integer = &'a PyLong;
    #[cfg(not(feature = "arbitrary_precision"))]
    type Integer = i64;
    fn as_integer<I: TryFrom<<Self as JsonNumber<'a>>::Integer>>(
        &'a self,
    ) -> Option<<PyAny as JsonNumber<'a>>::Integer> {
        if let Ok(integer) = self.downcast_exact::<PyLong>() {
            #[cfg(not(feature = "arbitrary_precision"))]
            {
                integer.extract().ok()
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                Some(integer)
            }
        } else {
            None
        }
    }
    fn as_float(&self) -> Option<f64> {
        if let Ok(integer) = self.downcast_exact::<PyLong>() {
            integer.extract().ok()
        } else if let Ok(float) = self.downcast::<PyFloat>() {
            Some(float.value())
        } else {
            None
        }
    }
}

impl Json for PyAny {
    type Object = PyDict;
    type Array = PyList;
    type String = str;
    type Number = PyAny;

    fn try_as_object(&self) -> Result<Option<&Self::Object>, JsonError> {
        Ok(self.downcast::<PyDict>().ok())
    }

    fn try_as_array(&self) -> Result<Option<&Self::Array>, JsonError> {
        Ok(self.downcast::<PyList>().ok())
    }

    fn try_as_string(&self) -> Result<Option<&Self::String>, JsonError> {
        if let Ok(pystring) = self.downcast::<PyString>() {
            Ok(pystring.to_str().map(Some)?)
        } else {
            Ok(None)
        }
    }

    fn try_as_number(&self) -> Result<Option<&Self::Number>, JsonError> {
        Ok(
            if self.downcast_exact::<PyLong>().is_ok() || self.downcast::<PyFloat>().is_ok() {
                Some(self)
            } else {
                None
            },
        )
    }

    fn try_as_boolean(&self) -> Result<Option<bool>, JsonError> {
        Ok(self.downcast_exact::<PyBool>().ok().map(PyBool::is_true))
    }

    fn try_as_null(&self) -> Result<Option<()>, JsonError> {
        Ok(self.downcast::<PyNone>().ok().map(|_| ()))
    }

    fn try_equal(&self, other: &Self) -> Result<bool, JsonError> {
        Ok(self.eq(other)?)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::tests::{
        assert_array_get, assert_array_iter, assert_as_array, assert_as_boolean, assert_as_null,
        assert_as_number_float, assert_as_number_integer, assert_as_object, assert_as_string,
        assert_object_get, assert_object_iter, CustomInteger,
    };

    use super::*;
    use test_case::test_case;

    #[cfg(feature = "arbitrary_precision")]
    impl<'a> TryFrom<&'a PyLong> for CustomInteger {
        type Error = ();

        fn try_from(value: &'a PyLong) -> Result<Self, Self::Error> {
            Ok(CustomInteger::new(
                value.extract::<i64>().expect("Invalid value"),
            ))
        }
    }

    fn pytest(f: impl Fn(Python)) {
        Python::with_gil(|py| {
            f(py);
            Ok::<(), ()>(())
        })
        .expect("Failed to run")
    }

    fn pyrun<'py>(py: Python<'py>, code: &str) -> &'py PyAny {
        py.eval(code, None, None).expect("Invalid Python code")
    }

    fn build_object(py: Python) -> &PyAny {
        pyrun(py, "{\"a\": 1, \"c\": \"d\"}")
    }

    fn build_array(py: Python) -> &PyAny {
        pyrun(py, "[1]")
    }

    fn build_string(py: Python) -> &PyAny {
        pyrun(py, "\"abc\"")
    }

    fn build_integer(py: Python) -> &PyAny {
        pyrun(py, "42")
    }

    fn build_float(py: Python) -> &PyAny {
        pyrun(py, "5.15")
    }

    fn build_bool(py: Python) -> &PyAny {
        pyrun(py, "True")
    }

    fn build_none(py: Python) -> &PyAny {
        pyrun(py, "None")
    }

    #[test]
    fn test_object_get() {
        pytest(|py| assert_object_get(build_object(py)));
    }

    #[test]
    fn test_object_iter() {
        pytest(|py| assert_object_iter(build_object(py)));
    }

    #[test]
    fn test_array_iter() {
        pytest(|py| assert_array_iter(build_array(py)));
    }

    #[test]
    fn test_array_get() {
        pytest(|py| assert_array_get(build_array(py)));
    }
    #[test_case(
        r#"{1: "2"}"#,
        "TypeError: 'int' object cannot be converted to 'PyString'"
    )]
    #[test_case(
        r#"{"\ud800": "2"}"#,
        "UnicodeEncodeError: 'utf-8' codec can't encode character '\\ud800' in position 0: surrogates not allowed"
    )]
    fn test_invalid_object_key(code: &str, expected: &str) {
        pytest(|py| {
            let dict = pyrun(py, code);
            let object = dict.as_object().expect("Should be an object");
            let mut iter = <PyDict as JsonObject>::iter(object);
            let (key, _) = iter.next().expect("Empty dict");
            let error = key.expect_err("Should be an error");
            assert_eq!(error.to_string(), expected);
            assert!(error.source().is_some());
        });
    }

    #[test_case(build_none, false)]
    #[test_case(build_bool, false)]
    #[test_case(build_float, false)]
    #[test_case(build_integer, false)]
    #[test_case(build_string, false)]
    #[test_case(build_array, false)]
    #[test_case(build_object, true)]
    fn test_as_object(factory: fn(Python<'_>) -> &PyAny, expected: bool) {
        pytest(|py| assert_as_object(factory(py), expected));
    }

    #[test_case(build_none, false)]
    #[test_case(build_bool, false)]
    #[test_case(build_float, false)]
    #[test_case(build_integer, false)]
    #[test_case(build_string, false)]
    #[test_case(build_array, true)]
    #[test_case(build_object, false)]
    fn test_as_array(factory: fn(Python<'_>) -> &PyAny, expected: bool) {
        pytest(|py| assert_as_array(factory(py), expected));
    }

    #[test_case(build_none, None)]
    #[test_case(build_bool, None)]
    #[test_case(build_float, None)]
    #[test_case(build_integer, None)]
    #[test_case(build_string, Some("abc"))]
    #[test_case(build_array, None)]
    #[test_case(build_object, None)]
    fn test_as_string(factory: fn(Python<'_>) -> &PyAny, expected: Option<&str>) {
        pytest(|py| assert_as_string(factory(py), expected));
    }

    #[test_case(build_none, None)]
    #[test_case(build_bool, None)]
    #[test_case(build_integer, Some(CustomInteger::new(42)))]
    #[test_case(build_float, None)]
    #[test_case(build_string, None)]
    #[test_case(build_array, None)]
    #[test_case(build_object, None)]
    fn test_as_number_integer(factory: fn(Python<'_>) -> &PyAny, expected: Option<CustomInteger>) {
        pytest(|py| assert_as_number_integer(factory(py), expected));
    }

    #[test_case(build_none, None)]
    #[test_case(build_bool, None)]
    #[test_case(build_integer, Some(42.0))]
    #[test_case(build_float, Some(5.15))]
    #[test_case(build_string, None)]
    #[test_case(build_array, None)]
    #[test_case(build_object, None)]
    fn test_as_number_float(factory: fn(Python<'_>) -> &PyAny, expected: Option<f64>) {
        pytest(|py| assert_as_number_float(factory(py), expected));
    }

    #[test_case(build_none, None)]
    #[test_case(build_bool, Some(true))]
    #[test_case(build_float, None)]
    #[test_case(build_integer, None)]
    #[test_case(build_string, None)]
    #[test_case(build_array, None)]
    #[test_case(build_object, None)]
    fn test_as_boolean(factory: fn(Python<'_>) -> &PyAny, expected: Option<bool>) {
        pytest(|py| assert_as_boolean(factory(py), expected));
    }

    #[test_case(build_none, Some(()))]
    #[test_case(build_bool, None)]
    #[test_case(build_float, None)]
    #[test_case(build_integer, None)]
    #[test_case(build_string, None)]
    #[test_case(build_array, None)]
    #[test_case(build_object, None)]
    fn test_as_null(factory: fn(Python<'_>) -> &PyAny, expected: Option<()>) {
        pytest(|py| assert_as_null(factory(py), expected));
    }

    #[test_case(build_none)]
    #[test_case(build_bool)]
    #[test_case(build_float)]
    #[test_case(build_integer)]
    #[test_case(build_string)]
    #[test_case(build_array)]
    #[test_case(build_object)]
    fn test_equal(factory: fn(Python<'_>) -> &PyAny) {
        pytest(|py| {
            let value = factory(py);
            assert!(value.equal(&value));
            let other = pyrun(py, "'something else'");
            assert!(!value.equal(other));
        });
    }
}
