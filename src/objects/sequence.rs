// Copyright (c) 2015 Daniel Grunwald
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::mem;
use ffi;
use python::{PythonObject, ToPythonPointer};
use objects::{PyObject, PyList, PyTuple};
use ffi::Py_ssize_t;
use err::{PyErr, PyResult, result_from_owned_ptr};

pub struct PySequence<'p>(PyObject<'p>);

pyobject_newtype!(PySequence, PySequence_Check);

impl <'p> PySequence<'p> {
    #[inline]
    pub fn size(&self) -> PyResult<'p, usize> {
        let v = unsafe { ffi::PySequence_Size(self.as_ptr()) };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(v as usize)
        }
    }

    #[inline]
    pub fn length(&self) -> usize {
        let v = unsafe { ffi::PySequence_Length(self.as_ptr()) };
        if v == -1 {
            panic!("Received an unexpected error finding the length of a PySequence")
        }
        v as usize
    }

    #[inline]
    pub fn concat(&self, other: &PySequence<'p>) -> PyResult<'p, PySequence> {
        let seq = try!(unsafe {
            let py = self.python();
            result_from_owned_ptr(py, ffi::PySequence_Concat(self.as_ptr(), other.as_ptr()))
        });
        Ok(PySequence(seq))
    }

    #[inline]
    pub fn repeat(&self, count: usize) -> PyResult<'p, PySequence> {
        let seq = try!(unsafe {
            let py = self.python();
            result_from_owned_ptr(py, ffi::PySequence_Repeat(self.as_ptr(), count as Py_ssize_t))
        });
        Ok(PySequence(seq))
    }

    #[inline]
    pub fn in_place_concat(&self, other: &PySequence<'p>) -> PyResult<'p, PySequence> {
        let seq = try!(unsafe {
            let py = self.python();
            result_from_owned_ptr(py, ffi::PySequence_InPlaceConcat(self.as_ptr(), other.as_ptr()))
        });
        Ok(PySequence(seq))
    }

    #[inline]
    pub fn in_place_repeat(&self, count: usize) -> PyResult<'p, PySequence> {
        let seq = try!(unsafe {
            let py = self.python();
            result_from_owned_ptr(py, 
                ffi::PySequence_InPlaceRepeat(self.as_ptr(), count as Py_ssize_t))
        });
        Ok(PySequence(seq))
    }

    #[inline]
    pub fn get_item(&self, index: usize) -> PyObject<'p> {
        assert!(index < self.length());
        unsafe {
            let py = self.python();
            PyObject::from_borrowed_ptr(py, ffi::PySequence_GetItem(self.as_ptr(), index as Py_ssize_t))
        }
    }

    #[inline]
    pub fn get_slice(&self, index: usize) -> PyResult<'p, PySequence> {
        let slice = try!(unsafe {
            let py = self.python();
            result_from_owned_ptr(py, 
                ffi::PySequence_GetItem(self.as_ptr(), index as Py_ssize_t))
        });
        Ok(PySequence(slice))
    }

    #[inline]
    pub fn set_item(&self, i: usize, v: &PyObject<'p>) -> PyResult<'p, ()> {
        let v = unsafe { 
            ffi::PySequence_SetItem(self.as_ptr(), i as Py_ssize_t, v.as_ptr()) 
        };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else{ 
            Ok(())
        }
    }

    #[inline]
    pub fn del_item(&self, i: usize) -> PyResult<'p, ()> {
        let v = unsafe { ffi::PySequence_DelItem(self.as_ptr(), i as Py_ssize_t) };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn set_slice(&self, i1: usize, i2: usize, v: &PyObject<'p>) -> PyResult<'p, ()> {
        let v = unsafe { 
            ffi::PySequence_SetSlice(self.as_ptr(), i1 as Py_ssize_t, i2 as Py_ssize_t, v.as_ptr()) 
        };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else{ 
            Ok(())
        }
    }

    #[inline]
    pub fn del_slice(&self, i1: i64, i2: i64) -> PyResult<'p, ()> {
        let v = unsafe { ffi::PySequence_DelSlice(self.as_ptr(), i1, i2) };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn count(&self, v: &PyObject<'p>) -> PyResult<'p, usize> {
        let v = unsafe { ffi::PySequence_Count(self.as_ptr(), v.as_ptr()) };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(v as usize)
        }
    }

    #[inline]
    pub fn contains(&self, v: &PyObject<'p>) -> PyResult<'p, bool> {
        let v = unsafe { ffi::PySequence_Contains(self.as_ptr(), v.as_ptr()) };
        match v {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(PyErr::fetch(self.python()))
        }
    }

    #[inline]
    pub fn index(&self, v: &PyObject<'p>) -> PyResult<'p, usize> {
        let v = unsafe { ffi::PySequence_Index(self.as_ptr(), v.as_ptr()) };
        if v == -1 {
            Err(PyErr::fetch(self.python()))
        } else {
            Ok(v as usize)
        }
    }

    #[inline]
    pub fn list(&self) -> PyResult<'p, PyList> {
        let v = try!(unsafe { 
            let py = self.python();
            result_from_owned_ptr(py, ffi::PySequence_List(self.as_ptr()))
        });
        Ok(unsafe { v.unchecked_cast_into::<PyList>() } )
    }

    #[inline]
    pub fn tuple(&self) -> PyResult<'p, PyTuple> {
        let v = try!(unsafe { 
            let py = self.python();
            result_from_owned_ptr(py, ffi::PySequence_Tuple(self.as_ptr()))
        });
        Ok(unsafe {v.unchecked_cast_into::<PyTuple>() } )
    }
}

pub struct PySequenceIterator<'p> {
    sequence : PySequence<'p>,
    index : usize
}

impl <'p> IntoIterator for PySequence<'p> {
    type Item = PyObject<'p>;
    type IntoIter = PySequenceIterator<'p>;

    fn into_iter(self) -> PySequenceIterator<'p> {
        PySequenceIterator{ sequence: self, index: 0 }
    }
}

impl <'a, 'p> IntoIterator for &'a PySequence<'p> {
    type Item = PyObject<'p>;
    type IntoIter = PySequenceIterator<'p>;

    #[inline]
    fn into_iter(self) -> PySequenceIterator<'p> {
        PySequenceIterator{ sequence: self.clone(), index: 0 }
    }
}

impl <'p> Iterator for PySequenceIterator<'p> {
    type Item = PyObject<'p>;

    #[inline]
    fn next(&mut self) -> Option<PyObject<'p>> {
        // can't report any errors in underlying size check so we panic.
        let len = self.sequence.length();
        if self.index < len {
            let item = self.sequence.get_item(self.index) ;
            self.index += 1;
            Some(item)
        } else { 
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std;
    use python::{Python, PythonObject};
    use conversion::ToPyObject;
    use objects::{PySequence, PyList, PyTuple};

    #[test]
    fn test_numbers_are_not_sequences() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = 42i32;
        assert!(v.to_py_object(py).into_object().cast_into::<PySequence>().is_err());
    }

    #[test]
    fn test_strings_are_sequences() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = "London Calling";
        assert!(v.to_py_object(py).into_object().cast_into::<PySequence>().is_ok());
    }
    #[test]
    fn test_seq_empty() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert_eq!(0, seq.length());
        assert_eq!(0, seq.size().unwrap());

        let needle = 7i32.to_py_object(py).into_object();
        assert_eq!(false, seq.contains(&needle).unwrap());
    }

    #[test]
    fn test_seq_contains() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 1, 2, 3, 5, 8];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert_eq!(6, seq.length());
        assert_eq!(6, seq.size().unwrap());

        let bad_needle = 7i32.to_py_object(py).into_object();
        assert_eq!(false, seq.contains(&bad_needle).unwrap());

        let good_needle = 8i32.to_py_object(py).into_object();
        assert_eq!(true, seq.contains(&good_needle).unwrap());

        let type_coerced_needle = 8f32.to_py_object(py).into_object();
        assert_eq!(true, seq.contains(&type_coerced_needle).unwrap());
    }

    #[test]
    fn test_seq_get_item() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 1, 2, 3, 5, 8];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert_eq!(1, seq.get_item(0).extract::<i32>().unwrap());
        assert_eq!(1, seq.get_item(1).extract::<i32>().unwrap());
        assert_eq!(2, seq.get_item(2).extract::<i32>().unwrap());
        assert_eq!(3, seq.get_item(3).extract::<i32>().unwrap());
        assert_eq!(5, seq.get_item(4).extract::<i32>().unwrap());
        assert_eq!(8, seq.get_item(5).extract::<i32>().unwrap());
        //assert!(seq.get_item(5).extract::<i32>().is_err()); // panics.
    }

    #[test]
    fn test_seq_index() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 1, 2, 3, 5, 8];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert_eq!(0, seq.index(&1i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(2, seq.index(&2i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(3, seq.index(&3i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(4, seq.index(&5i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(5, seq.index(&8i32.to_py_object(py).into_object()).unwrap());
        assert!(seq.index(&42i32.to_py_object(py).into_object()).is_err());
    }

    #[test]
    fn test_seq_count() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 1, 2, 3, 5, 8];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert_eq!(2, seq.count(&1i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(1, seq.count(&2i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(1, seq.count(&3i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(1, seq.count(&5i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(1, seq.count(&8i32.to_py_object(py).into_object()).unwrap());
        assert_eq!(0, seq.count(&42i32.to_py_object(py).into_object()).unwrap());
    }

    #[test]
    fn test_seq_iter() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 1, 2, 3, 5, 8];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        let mut idx = 0;
        for el in seq {
            assert_eq!(v[idx], el.extract::<i32>().unwrap());
            idx += 1;
        }
        assert_eq!(idx, v.len());
    }

    #[test]
    fn test_seq_into_iter() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 1, 2, 3, 5, 8];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        let mut idx = 0;
        for el in seq.into_iter() {
            assert_eq!(v[idx], el.extract::<i32>().unwrap());
            idx += 1;
        }
        assert_eq!(idx, v.len());
    }

    #[test]
    fn test_seq_strings() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec!["It", "was", "the", "worst", "of", "times"];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();

        let bad_needle = "blurst".to_py_object(py).into_object();
        assert_eq!(false, seq.contains(&bad_needle).unwrap());

        let good_needle = "worst".to_py_object(py).into_object();
        assert_eq!(true, seq.contains(&good_needle).unwrap());
    }

    #[test]
    fn test_seq_concat() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v : Vec<i32> = vec![1, 2, 3];
        let concat_v : Vec<i32> = vec![1, 2, 3, 1, 2, 3];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        let concat_seq = seq.concat(&seq).unwrap();
        assert_eq!(6, concat_seq.length());
        //assert_eq!(concat_v, concat_seq.into_object().extract::<Vec<i32>>().unwrap());
    }

    #[test]
    fn test_seq_concat_string() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = "string";
        let concat_v = "stringstring";
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        let concat_seq = seq.concat(&seq).unwrap();
        assert_eq!(12, concat_seq.length());
        //assert_eq!(concat_v, concat_seq.into_object().extract::<String>().unwrap());
    }

    #[test]
    fn test_seq_repeat() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec!["foo", "bar"];
        let repeated = vec!["foo", "bar", "foo", "bar", "foo", "bar"];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        let repeat_seq = seq.repeat(3).unwrap();
        assert_eq!(6, repeat_seq.length());
        //assert_eq!(repeated, repeat_seq.into_object().extract::<Vec<String>>().unwrap());
    }

    #[test]
    fn test_list_coercion() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec!["foo", "bar"];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert!(seq.list().is_ok());
    }

    #[test]
    fn test_strings_coerce_to_lists() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = "foo";
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert!(seq.list().is_ok());
    }

    #[test]
    fn test_tuple_coercion() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = ("foo", "bar");
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert!(seq.tuple().is_ok());
    }

    #[test]
    fn test_lists_coerce_to_tuples() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = vec!["foo", "bar"];
        let seq = v.to_py_object(py).into_object().cast_into::<PySequence>().unwrap();
        assert!(seq.tuple().is_ok());
    }
}
