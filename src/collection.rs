use crate::{
    bindings,
    bsonc::Bsonc,
    error::{BsoncError, Result},
    read_prefs::{ReadPrefs, ReadPrefsc},
};

use std::ptr;

#[derive(Debug)]
pub struct Collectionc {
    inner: *mut bindings::mongoc_collection_t,
}

pub trait Collection {
    type ReadPrefs: ReadPrefs + Sized;
    fn count(
        &self,
        filter: Option<bson::Document>,
        opts: Option<bson::Document>,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<i64>;
}

impl Collectionc {
    pub fn from_ptr(inner: *mut bindings::mongoc_collection_t) -> Self {
        Collectionc { inner }
    }
}

impl Collection for Collectionc {
    type ReadPrefs = ReadPrefsc;

    /// Counts the number of documents in a collection.
    ///
    /// From MongoDB Docs
    /// filter: A bson::Document containing the filter.
    /// opts: A bson::Document, None to ignore.
    /// read_prefs: A mongoc_read_prefs_t or None.
    /// opts may be None or a BSON document with additional command options:
    ///  readConcern: Construct a mongoc_read_concern_t and use mongoc_read_concern_append() to add the read concern to opts. See the example code for mongoc_client_read_command_with_opts(). Read concern requires MongoDB 3.2 or later, otherwise an error is returned.
    ///  sessionId: First, construct a mongoc_client_session_t with mongoc_client_start_session(). You can begin a transaction with mongoc_client_session_start_transaction(), optionally with a mongoc_transaction_opt_t that overrides the options inherited from collection, and use mongoc_client_session_append() to add the session to opts. See the example code for mongoc_client_session_t.
    ///  collation: Configure textual comparisons. See Setting Collation Order, and the MongoDB Manual entry on Collation. Collation requires MongoDB 3.2 or later, otherwise an error is returned.
    ///  serverId: To target a specific server, include an int32 “serverId” field. Obtain the id by calling mongoc_client_select_server(), then mongoc_server_description_id() on its return value.
    ///  skip: An int specifying how many documents matching the query should be skipped before counting.
    ///  limit: An int specifying the maximum number of documents to count.
    ///
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// let mut client = pool.pop();
    ///
    /// let collection = client.get_collection("test", "stuffs")?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(0, count);
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn count(
        &self,
        filter: Option<bson::Document>,
        opts: Option<bson::Document>,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<i64> {
        let bsonc_filter =
            filter.map_or_else(|| Ok(Bsonc::empty()), |d| Bsonc::from_document(&d))?;
        let bsonc_opts = opts.map_or_else(|| Ok(Bsonc::empty()), |d| Bsonc::from_document(&d))?;

        let mut reply = Bsonc::empty();
        let mut error = BsoncError::empty();
        let count = unsafe {
            bindings::mongoc_collection_count_documents(
                self.inner,
                bsonc_filter.as_ptr(),
                bsonc_opts.as_ptr(),
                read_prefs.unwrap_or_default().as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if count != -1 {
            Ok(count)
        } else {
            Err(error.into())
        }
    }
}

impl Drop for Collectionc {
    fn drop(&mut self) {
        dbg!("Collection drop start");
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_collection_destroy(self.inner);
            };
            self.inner = ptr::null_mut();
            dbg!(self);
        }
        dbg!("Collection drop done");
    }
}
