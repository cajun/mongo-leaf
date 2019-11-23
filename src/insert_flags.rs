//! Abstraction on top of the MongoDB connection read prefences.

use crate::bindings;

/// Describes how reads should be dispatched.
pub enum InsertFlag {
    // Specify no insert flags.
    InsertNone,
    // Continue inserting documents from the insertion set even if one insert fails.
    InsertContinueOnError,
}

fn flag_value(flags: &InsertFlag) -> bindings::mongoc_insert_flags_t {
    match *flags {
        InsertFlag::InsertNone => bindings::mongoc_insert_flags_t_MONGOC_INSERT_NONE,
        InsertFlag::InsertContinueOnError => {
            bindings::mongoc_insert_flags_t_MONGOC_INSERT_CONTINUE_ON_ERROR
        }
    }
}

pub struct InsertFlagsc {
    _inner: u32,
}

impl Default for InsertFlagsc {
    /// Get a new instance of the default read pref.
    fn default() -> Self {
        InsertFlagsc::new(&InsertFlag::InsertNone)
    }
}

pub trait InsertFlags {
    fn new(flag: &InsertFlag) -> Self;
}

impl InsertFlags for InsertFlagsc {
    /// Create a new empty insert flag.
    fn new(flag: &InsertFlag) -> Self {
        let _inner = flag_value(flag);
        InsertFlagsc { _inner }
    }
}
