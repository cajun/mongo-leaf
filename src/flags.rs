//! Flags to configure various MongoDB operations.

use crate::bindings;

use std::collections::BTreeSet;

/// Structure to hold flags for various flag types
pub struct Flags<T> {
    flags: BTreeSet<T>,
}

impl<T> Flags<T>
where
    T: Ord,
{
    /// Creare a new empty flags instance
    pub fn new() -> Flags<T> {
        Flags {
            flags: BTreeSet::new(),
        }
    }

    /// Add a flag to this instance
    pub fn add(&mut self, flag: T) {
        self.flags.insert(flag);
    }
}

impl<T> Default for Flags<T>
where
    T: Ord,
{
    fn default() -> Self {
        Flags::new()
    }
}

/// To provide the combined value of all flags.
pub trait FlagsValue {
    fn flags(&self) -> u32;
}

/*
/// Flags for insert operations
/// See: http://mongoc.org/libmongoc/current/mongoc_insert_flags_t.html
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum InsertFlag {
    ContinueOnError,
    NoValidate,
}

const INSERT_FLAG_NO_VALIDATE: u32 = 1 | 31; // MONGOC_INSERT_NO_VALIDATE defined in macro

impl FlagsValue for Flags<InsertFlag> {
    fn flags(&self) -> u32 {
        if self.flags.is_empty() {
            bindings::mongoc_insert_flags_t_MONGOC_INSERT_NONE
        } else {
            self.flags.iter().fold(0, {
                |flags, flag| {
                    flags
                        | match *flag {
                            InsertFlag::ContinueOnError => {
                                bindings::mongoc_insert_flags_t_MONGOC_INSERT_CONTINUE_ON_ERROR
                            }
                            InsertFlag::NoValidate => INSERT_FLAG_NO_VALIDATE,
                        }
                }
            })
        }
    }
}
*/

/// Flags for query operations
/// See: http://mongoc.org/libmongoc/current/mongoc_query_flags_t.html
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum QueryFlag {
    TailableCursor,
    SlaveOk,
    OplogReplay,
    NoCursorTimeout,
    AwaitData,
    Exhaust,
    Partial,
}

impl FlagsValue for Flags<QueryFlag> {
    fn flags(&self) -> u32 {
        if self.flags.is_empty() {
            bindings::mongoc_query_flags_t_MONGOC_QUERY_NONE
        } else {
            self.flags.iter().fold(0, {
                |flags, flag| {
                    flags
                        | match *flag {
                            QueryFlag::TailableCursor => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_TAILABLE_CURSOR
                            }
                            QueryFlag::SlaveOk => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_SLAVE_OK
                            }
                            QueryFlag::OplogReplay => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_OPLOG_REPLAY
                            }
                            QueryFlag::NoCursorTimeout => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_NO_CURSOR_TIMEOUT
                            }
                            QueryFlag::AwaitData => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_AWAIT_DATA
                            }
                            QueryFlag::Exhaust => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_EXHAUST
                            }
                            QueryFlag::Partial => {
                                bindings::mongoc_query_flags_t_MONGOC_QUERY_PARTIAL
                            }
                        }
                }
            })
        }
    }
}

/*
/// Flags for deletion operations
/// See: http://mongoc.org/libmongoc/current/mongoc_remove_flags_t.html
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum RemoveFlag {
    SingleRemove,
}

impl FlagsValue for Flags<RemoveFlag> {
    fn flags(&self) -> u32 {
        if self.flags.is_empty() {
            bindings::mongoc_remove_flags_t_MONGOC_REMOVE_NONE
        } else {
            bindings::mongoc_remove_flags_t_MONGOC_REMOVE_SINGLE_REMOVE
        }
    }
}

/// Flags for update operations
/// See: http://mongoc.org/libmongoc/current/mongoc_update_flags_t.html
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum UpdateFlag {
    Upsert,
    MultiUpdate,
}

impl FlagsValue for Flags<UpdateFlag> {
    fn flags(&self) -> u32 {
        if self.flags.is_empty() {
            bindings::mongoc_update_flags_t_MONGOC_UPDATE_NONE
        } else {
            self.flags.iter().fold(0, {
                |flags, flag| {
                    flags
                        | match *flag {
                            UpdateFlag::Upsert => {
                                bindings::mongoc_update_flags_t_MONGOC_UPDATE_UPSERT
                            }
                            UpdateFlag::MultiUpdate => {
                                bindings::mongoc_update_flags_t_MONGOC_UPDATE_MULTI_UPDATE
                            }
                        }
                }
            })
        }
    }
}
*/
