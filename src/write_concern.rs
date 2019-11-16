//! Abstraction on top of the MongoDB connection write concern.

use crate::bindings;
use std::ffi::CString;
use std::ptr;

/// Possible write concern levels, only default is supported at the moment.
pub enum WriteConcernLevel {
    /// By default, writes block awaiting acknowledgment from MongoDB. Acknowledged write concern allows clients to catch network, duplicate key, and other errors.
    Blocking,
    // With this write concern, MongoDB does not acknowledge the receipt of write operation. Unacknowledged is similar to errors ignored; however, mongoc attempts to receive and handle network errors when possible.
    WriteUnacknowledged,
    // Block until a write has been propagated to a majority of the nodes in the replica set.
    Majority,
    // Block until a write has been propagated to at least n nodes in the replica set.
    AtLeastNumberOfNodes(i32),
    // Block until a write has been propagated to at least n nodes in the replica set.
    WithTag(String),
}

/// This tells the driver what level of acknowledgment to await from the server.
/// The default, `Default`, is right for the great majority of applications.
pub struct WriteConcernc {
    inner: *mut bindings::mongoc_write_concern_t,
}

pub trait WriteConcern {
    fn new(level: WriteConcernLevel, timeout: Option<i64>) -> Self;
    fn as_ptr(&self) -> *const bindings::mongoc_write_concern_t {
        ptr::null()
    }
}

impl Default for WriteConcernc {
    /// Get the default write concern
    fn default() -> Self {
        WriteConcernc::new(WriteConcernLevel::Blocking, None)
    }
}

impl WriteConcern for WriteConcernc {
    /// Create a new write concern
    fn new(level: WriteConcernLevel, timeout: Option<i64>) -> Self {
        let inner = unsafe {
            let ptr = bindings::mongoc_write_concern_new();
            match level {
                WriteConcernLevel::Blocking => ptr,
                WriteConcernLevel::Majority => {
                    bindings::mongoc_write_concern_set_wmajority(ptr, 0);
                    if let Some(t) = timeout {
                        bindings::mongoc_write_concern_set_wtimeout_int64(ptr, t);
                    }
                    ptr
                }
                WriteConcernLevel::WriteUnacknowledged => {
                    bindings::mongoc_write_concern_set_w(ptr, 0);
                    if let Some(t) = timeout {
                        bindings::mongoc_write_concern_set_wtimeout_int64(ptr, t);
                    }
                    ptr
                }
                WriteConcernLevel::AtLeastNumberOfNodes(w) => {
                    bindings::mongoc_write_concern_set_w(ptr, w);
                    if let Some(t) = timeout {
                        bindings::mongoc_write_concern_set_wtimeout_int64(ptr, t);
                    }
                    ptr
                }
                WriteConcernLevel::WithTag(tag) => {
                    // Ok to blow up here
                    let t = CString::new(tag).expect("should be valid tag");
                    bindings::mongoc_write_concern_set_wtag(ptr, t.as_ptr());
                    if let Some(t) = timeout {
                        bindings::mongoc_write_concern_set_wtimeout_int64(ptr, t);
                    }
                    ptr
                }
            }
        };
        assert!(!inner.is_null());
        WriteConcernc { inner: inner }
    }

    fn as_ptr(&self) -> *const bindings::mongoc_write_concern_t {
        assert!(!self.inner.is_null());
        self.inner
    }
}

impl Drop for WriteConcernc {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        unsafe {
            bindings::mongoc_write_concern_destroy(self.inner);
        }
    }
}
