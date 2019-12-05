use crate::{
    bsonc::Bsonc,
    flags::{Flags, InsertFlag, QueryFlag, RemoveFlag, UpdateFlag},
    read_prefs::ReadMode,
    write_concern::WriteConcernLevel,
};
use bson::Document;
use std::time::Duration;

///  to configure an aggregate operation.
pub struct Aggregate {
    /// Flags to use
    pub query_flags: Flags<QueryFlag>,
    ///  for the aggregate
    pub options: Option<Document>,
    /// Read prefs to use
    pub read_prefs: Option<ReadMode>,
}

impl Default for Aggregate {
    /// Default options that are used if no options are specified
    /// when aggregating.
    fn default() -> Self {
        Aggregate {
            query_flags: Flags::new(),
            options: None,
            read_prefs: None,
        }
    }
}

///  to configure a bulk operation.
pub struct BulkOperation {
    /// If the operations must be performed in order
    pub ordered: bool,
    /// `WriteConcern` to use
    pub write_concern: WriteConcernLevel,
}

impl Default for BulkOperation {
    /// Default options that are used if no options are specified
    /// when creating a `BulkOperation`.
    fn default() -> Self {
        BulkOperation {
            ordered: false,
            write_concern: WriteConcernLevel::Blocking,
        }
    }
}

///  to configure a find and modify operation.
pub struct FindAndModify {
    /// Sort order for the query
    pub sort: Option<Document>,
    /// If the new version of the document should be returned
    pub new: bool,
    /// The fields to return
    pub fields: Option<Document>,
}

impl Default for FindAndModify {
    /// Default options used if none are provided.
    fn default() -> Self {
        FindAndModify {
            sort: None,
            new: false,
            fields: None,
        }
    }
}

impl FindAndModify {
    pub fn fields_bsonc(&self) -> Option<Bsonc> {
        self.fields
            .as_ref()
            .map(|f| Bsonc::from_document(&f).expect("FindAndModify should be valid"))
    }
}

///  to configure a count operation.
pub struct Count {
    /// The query flags to use
    pub query_flags: Flags<QueryFlag>,
    /// Number of results to skip, zero to ignore
    pub skip: u32,
    /// Limit to the number of results, zero to ignore
    pub limit: u32,
    /// Optional extra keys to add to the count
    pub opts: Option<Document>,
    /// Read prefs to use
    pub read_prefs: Option<ReadMode>,
}

impl Count {}

impl Default for Count {
    /// Default options used if none are provided.
    fn default() -> Self {
        Count {
            query_flags: Flags::new(),
            skip: 0,
            limit: 0,
            opts: None,
            read_prefs: None,
        }
    }
}

///  to configure an insert operation.
pub struct Insert {
    /// Flags to use
    pub insert_flags: Flags<InsertFlag>,
    /// Write concern to use
    pub write_concern: WriteConcernLevel,
}

impl Default for Insert {
    /// Default options used if none are provided.
    fn default() -> Self {
        Insert {
            insert_flags: Flags::new(),
            write_concern: WriteConcernLevel::Blocking,
        }
    }
}

///  to configure a remove operation.
pub struct Remove {
    /// Flags to use
    pub remove_flags: Flags<RemoveFlag>,
    /// Write concern to use
    pub write_concern: WriteConcernLevel,
}

impl Default for Remove {
    /// Default options used if none are provided.
    fn default() -> Self {
        Remove {
            remove_flags: Flags::new(),
            write_concern: WriteConcernLevel::Blocking,
        }
    }
}

///  to configure an update operation.
pub struct Update {
    /// Flags to use
    pub update_flags: Flags<UpdateFlag>,
    /// Write concern to use
    pub write_concern: WriteConcernLevel,
}

impl Default for Update {
    /// Default options used if none are provided.
    fn default() -> Self {
        Update {
            update_flags: Flags::new(),
            write_concern: WriteConcernLevel::Blocking,
        }
    }
}

///  to configure a tailing query.
pub struct Tail {
    /// Duration to wait before checking for new results
    pub wait_duration: Duration,
    /// Maximum number of retries if there is an error
    pub max_retries: u32,
}

impl Default for Tail {
    /// Default options used if none are provided.
    fn default() -> Self {
        Tail {
            wait_duration: Duration::from_millis(500),
            max_retries: 5,
        }
    }
}
