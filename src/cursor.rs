use crate::{
    bindings,
    bsonc::Bsonc,
    error::{BsoncError, MongoError, Result},
    host::{Host, Hostc},
};
use std::ptr;

#[derive(Debug)]
pub struct Cursorc {
    inner: *mut bindings::mongoc_cursor_t,
}

pub trait Cursor {
    type Host: Host;
    fn get_hosts(&self) -> Option<Vec<Self::Host>>;
}

impl Cursorc {
    pub fn from_ptr(inner: *mut bindings::mongoc_cursor_t) -> Self {
        Cursorc { inner }
    }

    fn get_error(&self) -> Option<BsoncError> {
        assert!(!self.inner.is_null());

        unsafe {
            let mut error = BsoncError::empty();
            bindings::mongoc_cursor_error(self.inner, error.mut_inner());
            if error.is_empty() {
                None
            } else {
                Some(error)
            }
        }
    }
}

impl Cursor for Cursorc {
    type Host = Hostc;

    fn get_hosts(&self) -> Option<Vec<Self::Host>> {
        assert!(!self.inner.is_null());

        unsafe {
            let mut ptr = ptr::null_mut();
            bindings::mongoc_cursor_get_host(self.inner, ptr);
            if ptr.is_null() {
                None
            } else {
                let hosts = Hostc::host_list_from_ptr(ptr);
                Some(hosts)
            }
        }
    }
}

impl Iterator for Cursorc {
    type Item = Result<bson::Document>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ptr = ptr::null_mut();
        let is_done_or_error = unsafe { bindings::mongoc_cursor_next(self.inner, ptr) };

        if let Some(err) = self.get_error() {
            Some(Err(err.into()))
        } else if !is_done_or_error {
            let mut bson_ptr = ptr as *mut bindings::bson_t;
            Some(Bsonc::from_ptr(bson_ptr).as_document())
        } else {
            None
        }
    }
}

impl Drop for Cursorc {
    fn drop(&mut self) {
        unsafe {
            bindings::mongoc_cursor_destroy(self.inner);
        }
    }
}
