use crate::{
    bindings,
    collection::{Collection, Collectionc},
    error::{BsoncError, Result},
};
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Debug)]
pub struct Databasec {
    inner: *mut bindings::mongoc_database_t,
}

pub trait Database {
    type Collection: Collection;

    fn name(&self) -> String;
    fn as_mut_ptr(&self) -> *mut bindings::mongoc_database_t {
        ptr::null_mut()
    }

    fn destroy(&self) -> Result<bool>;
    fn get_collection(&self, name: impl Into<String>) -> Self::Collection;
}

impl Databasec {
    pub fn new(inner: *mut bindings::mongoc_database_t) -> Self {
        Databasec { inner }
    }
}

impl Database for Databasec {
    type Collection = Collectionc;

    fn name(&self) -> String {
        unsafe {
            let cstr = CStr::from_ptr(bindings::mongoc_database_get_name(self.inner));
            String::from_utf8_lossy(cstr.to_bytes()).to_string()
        }
    }

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_database_t {
        self.inner
    }

    fn get_collection(&self, name: impl Into<String>) -> Self::Collection {
        let ptr = unsafe {
            let coll_str = CString::new(name.into()).expect("Valid collection name");

            bindings::mongoc_database_get_collection(self.inner, coll_str.as_ptr())
        };

        Collectionc::from_ptr(ptr)
    }

    fn destroy(&self) -> Result<bool> {
        let mut error = BsoncError::empty();
        let success = unsafe { bindings::mongoc_database_drop(self.inner, error.as_mut_ptr()) };

        if error.is_empty() {
            Ok(success)
        } else {
            Err(error.into())
        }
    }
}

impl Drop for Databasec {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_database_destroy(self.inner);
            }
            self.inner = ptr::null_mut();
        }
    }
}
