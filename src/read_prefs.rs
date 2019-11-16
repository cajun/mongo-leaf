//! Abstraction on top of the MongoDB connection read prefences.

use crate::bindings;
use std::ptr;

/// Describes how reads should be dispatched.
pub enum ReadMode {
    /// Default mode. All operations read from the current replica set primary.
    Primary,
    /// All operations read from among the nearest secondary members of the replica set.
    Secondary,
    /// In most situations, operations read from the primary but if it is unavailable, operations read from secondary members.
    PrimaryPreferred,
    /// In most situations, operations read from among the nearest secondary members, but if no secondaries are available, operations read from the primary.
    SecondaryPreferred,
    /// Operations read from among the nearest members of the replica set, irrespective of the member’s type.
    Nearest,
}

fn read_mode_value(read_mode: &ReadMode) -> bindings::mongoc_read_mode_t {
    match *read_mode {
        ReadMode::Primary => bindings::mongoc_read_mode_t_MONGOC_READ_PRIMARY,
        ReadMode::Secondary => bindings::mongoc_read_mode_t_MONGOC_READ_SECONDARY,
        ReadMode::PrimaryPreferred => bindings::mongoc_read_mode_t_MONGOC_READ_PRIMARY_PREFERRED,
        ReadMode::SecondaryPreferred => {
            bindings::mongoc_read_mode_t_MONGOC_READ_SECONDARY_PREFERRED
        }
        ReadMode::Nearest => bindings::mongoc_read_mode_t_MONGOC_READ_NEAREST,
    }
}

pub struct ReadPrefsc {
    inner: *mut bindings::mongoc_read_prefs_t,
}

impl Default for ReadPrefsc {
    /// Get a new instance of the default read pref.
    fn default() -> Self {
        ReadPrefs::new(&ReadMode::Primary)
    }
}

pub trait ReadPrefs {
    fn new(read_mode: &ReadMode) -> Self;
    fn as_ptr(&self) -> *const bindings::mongoc_read_prefs_t {
        ptr::null()
    }
}

impl ReadPrefs for ReadPrefsc {
    /// Create a new empty read prefs.
    fn new(read_mode: &ReadMode) -> ReadPrefsc {
        let read_mode_value = read_mode_value(read_mode);
        let inner = unsafe { bindings::mongoc_read_prefs_new(read_mode_value) };
        assert!(!inner.is_null());
        ReadPrefsc { inner }
    }

    fn as_ptr(&self) -> *const bindings::mongoc_read_prefs_t {
        assert!(!self.inner.is_null());
        self.inner
    }
}

impl Drop for ReadPrefsc {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        unsafe {
            bindings::mongoc_read_prefs_destroy(self.inner);
        }
    }
}
