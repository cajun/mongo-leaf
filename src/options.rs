use crate::{
    bsonc::Bsonc,
    error::Result,
    flags::{Flags, FlagsValue, QueryFlag},
    read_prefs::ReadMode,
    write_concern::WriteConcernLevel,
};
use bson::Document;
use std::time::Duration;

///  to configure an aggregate operation.
pub struct Aggregate {
    /// Flags to use
    pub query_flags: Flags<QueryFlag>,
    pub batch_size: Option<i32>,
}

impl Default for Aggregate {
    /// Default options that are used if no options are specified
    /// when aggregating.
    fn default() -> Self {
        Aggregate {
            query_flags: Flags::new(),
            batch_size: None,
        }
    }
}

impl Aggregate {
    pub(crate) fn into_mongoc(self) -> Result<Bsonc> {
        if let Some(size) = self.batch_size {
            let d = doc! {
                "batchSize": size,
            };
            Bsonc::from_document(&d)
        } else {
            Ok(Bsonc::empty())
        }
    }

    pub(crate) fn to_mongoc(&self) -> Result<Bsonc> {
        if let Some(size) = self.batch_size {
            let d = doc! {
                "batchSize": size,
            };
            Bsonc::from_document(&d)
        } else {
            Ok(Bsonc::empty())
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
    /// Number of results to skip, zero to ignore
    pub skip: u32,
    /// Limit to the number of results, zero to ignore
    pub limit: u32,
}

impl Count {
    pub(crate) fn into_mongoc(self) -> Result<Bsonc> {
        let d = doc! {
            "skip": self.skip,
            "limit": self.limit,
        };

        Bsonc::from_document(&d)
    }
}

impl Default for Count {
    /// Default options used if none are provided.
    fn default() -> Self {
        Count { skip: 0, limit: 0 }
    }
}

///  to configure an insert operation.
pub struct Insert {
    pub ordered: bool,
    pub bypass_document_validation: bool,
}

impl Default for Insert {
    /// Default options used if none are provided.
    fn default() -> Self {
        Insert {
            ordered: true,
            bypass_document_validation: false,
        }
    }
}

impl Insert {
    pub(crate) fn into_mongoc(self) -> Result<Bsonc> {
        let d = doc! {
            "ordered": self.ordered,
            "bypassDocumentValidation": self.bypass_document_validation,
        };

        Bsonc::from_document(&d)
    }
}

/// For future use when Transactions are working
pub struct Remove;

impl Default for Remove {
    /// Default options used if none are provided.
    fn default() -> Self {
        Remove {}
    }
}

impl Remove {
    pub(crate) fn into_mongoc(self) -> Result<Bsonc> {
        Ok(Bsonc::empty())
    }
}

///  to configure an update operation.
pub struct Update {
    pub upsert: bool,
    pub bypass_document_validation: bool,
}

impl Default for Update {
    /// Default options used if none are provided.
    fn default() -> Self {
        Update {
            upsert: false,
            bypass_document_validation: false,
        }
    }
}

impl Update {
    pub(crate) fn into_mongoc(self) -> Result<Bsonc> {
        let d = doc! {
            "upsert": self.upsert,
            "bypassDocumentValidation": self.bypass_document_validation,
        };

        Bsonc::from_document(&d)
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
