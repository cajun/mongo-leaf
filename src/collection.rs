use crate::{
    bindings,
    bsonc::Bsonc,
    cursor::{Cursor, Cursorc},
    error::{BsoncError, Result},
    insert_flags::{InsertFlags, InsertFlagsc},
    read_prefs::{ReadPrefs, ReadPrefsc},
    write_concern::{WriteConcern, WriteConcernc},
};

use std::ptr;

#[derive(Debug)]
pub struct Collectionc {
    inner: *mut bindings::mongoc_collection_t,
}

pub trait Collection {
    type InsertFlags: InsertFlags + Sized;
    type ReadPrefs: ReadPrefs + Sized;
    type WriteConcern: WriteConcern + Sized;
    type Cursor: Cursor;

    fn count(
        &self,
        filter: Option<bson::Document>,
        opts: Option<bson::Document>,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<i64>;

    fn insert_one(
        &self,
        doc: bson::Document,
        // TODO: Update with the full opts. Many more flags
        // write_concern: Option<Self::WriteConcern>,
    ) -> Result<bson::Document>;

    fn insert_many(
        &self,
        docs: Vec<bson::Document>,
        // TODO: Update with the full opts. Many more flags
        // write_concern: Option<Self::WriteConcern>,
    ) -> Result<bson::Document>;

    fn delete(
        &self,
        selector: bson::Document,
        // TODO: Update with the full opts. Many more flags
        // write_concern: Option<Self::WriteConcern>,
    ) -> Result<bson::Document>;

    fn update(
        &self,
        selector: bson::Document,
        update: bson::Document,
        // TODO: Update with the full opts. Many more flags
        // write_concern: Option<Self::WriteConcern>,
    ) -> Result<bson::Document>;

    fn find(
        &self,
        filter: bson::Document,
        opts: Option<bson::Document>,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Self::Cursor;

    fn destroy(&self) -> Result<bool>;
}

impl Collectionc {
    pub fn from_ptr(inner: *mut bindings::mongoc_collection_t) -> Self {
        Collectionc { inner }
    }
}

impl Collection for Collectionc {
    type InsertFlags = InsertFlagsc;
    type ReadPrefs = ReadPrefsc;
    type WriteConcern = WriteConcernc;
    type Cursor = Cursorc;

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
    /// # use std::panic;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
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

        let reply = Bsonc::empty();
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

    /// Inserts one doc the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// let result = collection.insert_one(doc)?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(1, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn insert_one(&self, doc: bson::Document) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();
        let opts = Bsonc::empty();

        let success = unsafe {
            bindings::mongoc_collection_insert_one(
                self.inner,
                Bsonc::from_document(&doc)?.as_ptr(),
                opts.as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if success {
            reply.as_document()
        } else {
            Err(error.into())
        }
    }

    /// Inserts many docs the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let docs = vec![
    ///     doc!{"name": "first"},
    ///     doc!{"name": "second"},
    ///     doc!{"name": "third"},
    ///     doc!{"name": "fourth"},
    /// ];
    /// let result = collection.insert_many(docs)?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(4, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn insert_many(&self, docs: Vec<bson::Document>) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();
        let opts = Bsonc::empty();

        let bsonc: Vec<Bsonc> = docs
            .iter()
            .map(|d| Bsonc::from_document(&d).expect("should be valid doc"))
            .collect();

        let ptrs: Vec<*const bindings::bson_t> = bsonc.iter().map(|b| b.as_ptr()).collect();

        let success = unsafe {
            bindings::mongoc_collection_insert_many(
                self.inner,
                ptrs.as_ptr() as *mut *const bindings::bson_t,
                docs.len(),
                opts.as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if success {
            reply.as_document()
        } else {
            Err(error.into())
        }
    }

    /// Deletes one doc the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(1, count);
    /// let selector = doc!{"name": "omg"};
    /// collection.delete(selector)?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn delete(&self, doc: bson::Document) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();
        let opts = Bsonc::empty();

        let success = unsafe {
            bindings::mongoc_collection_delete_many(
                self.inner,
                Bsonc::from_document(&doc)?.as_ptr(),
                opts.as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if success {
            reply.as_document()
        } else {
            Err(error.into())
        }
    }

    /// Updates one doc the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(1, count);
    /// let selector = doc!{"name": "omg"};
    /// collection.update(selector, doc!{"$set": {"name": "foo"}})?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(1, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn update(&self, selector: bson::Document, update: bson::Document) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();
        let opts = Bsonc::empty();

        let success = unsafe {
            bindings::mongoc_collection_update_many(
                self.inner,
                Bsonc::from_document(&selector)?.as_ptr(),
                Bsonc::from_document(&update)?.as_ptr(),
                opts.as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if success {
            reply.as_document()
        } else {
            Err(error.into())
        }
    }

    /// Finds docs the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let doc = doc!{"name": "foo"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None, None, None)?;
    /// assert_eq!(2, count);
    ///
    /// let maybe: Result<Vec<bson::Document>> = collection.find(doc!{"name": "foo"}, None, None).collect();
    ///
    /// assert!(maybe.is_ok());
    /// let records = maybe.unwrap();
    ///
    /// assert_eq!(1, records.len());
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn find(
        &self,
        filter: bson::Document,
        opts: Option<bson::Document>,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Self::Cursor {
        let bson_filter = Bsonc::from_document(&filter).expect("should be valid");
        let bson_opts = opts.map_or_else(Bsonc::empty, |o| {
            Bsonc::from_document(&o).expect("should be valid")
        });

        let ptr = unsafe {
            bindings::mongoc_collection_find_with_opts(
                self.inner,
                bson_filter.as_ptr(),
                bson_opts.as_ptr(),
                read_prefs.unwrap_or_default().as_ptr(),
            )
        };

        Cursorc::from_ptr(ptr)
    }

    /// Drops collection
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// collection.destroy();
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn destroy(&self) -> Result<bool> {
        let mut error = BsoncError::empty();
        let success = unsafe { bindings::mongoc_collection_drop(self.inner, error.as_mut_ptr()) };

        if error.is_empty() {
            Ok(success)
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
