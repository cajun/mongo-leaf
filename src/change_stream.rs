use crate::{
    bindings,
    bsonc::Bsonc,
    error::{BsoncError, Result},
};
use std::ptr;

#[derive(Debug)]
pub struct ChangeStreamc {
    inner: *mut bindings::mongoc_change_stream_t,
}

pub trait ChangeStream {}

impl ChangeStreamc {
    pub fn from_ptr(inner: *mut bindings::mongoc_change_stream_t) -> Self {
        ChangeStreamc { inner }
    }

    pub fn get_error(&self) -> Option<BsoncError> {
        assert!(!self.inner.is_null(), "change stream ptr null");

        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();

        let has_c_error = unsafe {
            bindings::mongoc_change_stream_error_document(
                self.inner,
                error.as_mut_ptr(),
                &mut reply.as_ptr(),
            )
        };

        if !has_c_error {
            return None;
        }

        if error.is_empty() {
            None
        } else {
            Some(error)
        }
    }
}

impl ChangeStream for ChangeStreamc {}

impl Iterator for ChangeStreamc {
    type Item = Result<bson::Document>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bson_ptr: *const bindings::bson_t = ptr::null_mut();

        let success = unsafe { bindings::mongoc_change_stream_next(self.inner, &mut bson_ptr) };

        if let Some(err) = self.get_error() {
            Some(Err(err.into()))
        } else if success {
            dbg!("success?");
            let bsonc = Bsonc::from_ptr(bson_ptr);
            Some(bsonc.as_document())
        } else {
            dbg!("none?");
            None
        }
    }
}

impl Drop for ChangeStreamc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_change_stream_destroy(self.inner);
            };
            self.inner = ptr::null_mut();
        }
    }
}
