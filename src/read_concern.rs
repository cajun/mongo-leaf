//! Abstraction on top of the MongoDB connection read concern.

use crate::{
    bindings,
    bsonc::Bsonc,
    error::{MongoError, Result},
};
use std::ffi::CString;
use std::fmt;
use std::ptr;

/// Possible read concern levels, only default is supported at the moment.
#[derive(Copy, Clone, Debug)]
pub enum ReadConcernLevel {
    // Default
    Local,
    Majority,
    Linearizable,
    Available,
    Snapshot,
}

impl fmt::Display for ReadConcernLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadConcernLevel::Local => write!(f, "Local: Mongo 3.2"),
            ReadConcernLevel::Majority => write!(f, "Majority: Mongo 3.2"),
            ReadConcernLevel::Linearizable => write!(f, "Linearizable: Mongo 3.4"),
            ReadConcernLevel::Available => write!(f, "Availabe: Mongo 3.6"),
            ReadConcernLevel::Snapshot => write!(f, "Snapshot: Mongo 4.0"),
        }
    }
}

impl ReadConcernLevel {
    fn to_mongoc(self) -> CString {
        unsafe {
            match self {
                ReadConcernLevel::Local => {
                    CString::from_vec_unchecked(bindings::MONGOC_READ_CONCERN_LEVEL_LOCAL.to_vec())
                }
                ReadConcernLevel::Majority => CString::from_vec_unchecked(
                    bindings::MONGOC_READ_CONCERN_LEVEL_MAJORITY.to_vec(),
                ),
                ReadConcernLevel::Linearizable => CString::from_vec_unchecked(
                    bindings::MONGOC_READ_CONCERN_LEVEL_LINEARIZABLE.to_vec(),
                ),
                ReadConcernLevel::Available => CString::from_vec_unchecked(
                    bindings::MONGOC_READ_CONCERN_LEVEL_AVAILABLE.to_vec(),
                ),
                ReadConcernLevel::Snapshot => CString::from_vec_unchecked(
                    bindings::MONGOC_READ_CONCERN_LEVEL_SNAPSHOT.to_vec(),
                ),
            }
        }
    }
}

/// This tells the driver what level of acknowledgment to await from the server.
/// The default, `Default`, is right for the great majority of applications.
pub struct ReadConcernc {
    inner: *mut bindings::mongoc_read_concern_t,
}

impl ReadConcernc {
    pub fn append(&self, opts: &mut Bsonc) -> bool {
        unsafe { bindings::mongoc_read_concern_append(self.inner, opts.as_mut_ptr()) }
    }
}

pub trait ReadConcern {
    fn set_level(&self, level: ReadConcernLevel) -> Result<&Self>;
    fn as_ptr(&self) -> *const bindings::mongoc_read_concern_t {
        ptr::null()
    }
}

impl Default for ReadConcernc {
    /// Get the default read concern
    fn default() -> Self {
        ReadConcernc::new(ReadConcernLevel::Local).expect("local read concern not working")
    }
}

impl ReadConcern for ReadConcernc {
    fn set_level(&self, level: ReadConcernLevel) -> Result<&Self> {
        let success = unsafe {
            bindings::mongoc_read_concern_set_level(self.inner, level.to_mongoc().as_ptr())
        };

        if success {
            Ok(self)
        } else {
            Err(MongoError::InvalidReadConcern(level).into())
        }
    }

    fn as_ptr(&self) -> *const bindings::mongoc_read_concern_t {
        assert!(!self.inner.is_null());
        self.inner
    }
}

impl ReadConcernc {
    /// Create a new read concern
    fn new(level: ReadConcernLevel) -> Result<ReadConcernc> {
        let inner = unsafe { bindings::mongoc_read_concern_new() };
        assert!(!inner.is_null());

        let success =
            unsafe { bindings::mongoc_read_concern_set_level(inner, level.to_mongoc().as_ptr()) };

        if success {
            Ok(ReadConcernc { inner })
        } else {
            Err(MongoError::InvalidReadConcern(level).into())
        }
    }
}

impl Drop for ReadConcernc {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        unsafe {
            bindings::mongoc_read_concern_destroy(self.inner);
        }
    }
}
