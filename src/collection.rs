use crate::{
    bindings,
    bsonc::Bsonc,
    change_stream::{ChangeStream, ChangeStreamc},
    cursor::{Cursor, Cursorc},
    error::{BsoncError, Result},
    flags::FlagsValue,
    options::{Aggregate, Count, FindAndModify, Insert, Remove, Update},
    read_prefs::{ReadPrefs, ReadPrefsc},
};

use std::ptr;

#[derive(Debug)]
pub struct Collectionc {
    inner: *mut bindings::mongoc_collection_t,
}

pub trait Collection {
    type Cursor: Cursor;
    type ChangeStream: ChangeStream;

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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    ///
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let count = collection.count(None)?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn count(&self, filter: Option<bson::Document>) -> Result<i64>;
    fn count_with_opts(&self, filter: Option<bson::Document>, opts: Option<Count>) -> Result<i64>;

    fn insert_one(&self, doc: bson::Document) -> Result<bson::Document>;

    fn insert_one_with_opts(
        &self,
        doc: bson::Document,
        opts: Option<Insert>,
    ) -> Result<bson::Document>;

    fn insert_many(&self, docs: Vec<bson::Document>) -> Result<bson::Document>;

    fn insert_many_with_opts(
        &self,
        docs: Vec<bson::Document>,
        opts: Option<Insert>,
    ) -> Result<bson::Document>;

    fn delete(&self, selector: bson::Document) -> Result<bson::Document>;

    fn delete_with_opts(
        &self,
        selector: bson::Document,
        opts: Option<Remove>,
    ) -> Result<bson::Document>;

    fn update(&self, selector: bson::Document, update: bson::Document) -> Result<bson::Document>;

    fn update_with_opts(
        &self,
        selector: bson::Document,
        update: bson::Document,
        opts: Option<Update>,
    ) -> Result<bson::Document>;

    fn find(&self, filter: bson::Document) -> Self::Cursor;
    fn find_with_opts(&self, filter: bson::Document, opts: Option<FindAndModify>) -> Self::Cursor;

    fn aggregate(&self, pipeline: bson::Document) -> Self::Cursor;

    fn aggregate_with_opts(
        &self,
        pipeline: bson::Document,
        opts: Option<Aggregate>,
    ) -> Self::Cursor;

    fn drop_collection(&self) -> Result<bool>;

    fn watch(
        &self,
        pipeline: Option<bson::Document>,
        opts: Option<bson::Document>,
    ) -> Result<Self::ChangeStream>;
}

impl Collectionc {
    pub(crate) fn from_ptr(inner: *mut bindings::mongoc_collection_t) -> Self {
        Collectionc { inner }
    }
}

impl Collection for Collectionc {
    type Cursor = Cursorc;
    type ChangeStream = ChangeStreamc;

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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    ///
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let count = collection.count(None)?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn count(&self, filter: Option<bson::Document>) -> Result<i64> {
        self.count_with_opts(filter, None)
    }

    /// Inserts one doc the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// let result = collection.insert_one(doc)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn insert_one(&self, doc: bson::Document) -> Result<bson::Document> {
        self.insert_one_with_opts(doc, None)
    }

    /// Inserts many docs the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
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
    /// let count = collection.count(None)?;
    /// assert_eq!(4, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn insert_many(&self, docs: Vec<bson::Document>) -> Result<bson::Document> {
        self.insert_many_with_opts(docs, None)
    }

    /// Deletes one doc the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    /// let selector = doc!{"name": "omg"};
    /// collection.delete(selector)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn delete(&self, doc: bson::Document) -> Result<bson::Document> {
        self.delete_with_opts(doc, None)
    }

    /// Updates one doc the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    /// let selector = doc!{"name": "omg"};
    /// collection.update(selector, doc!{"$set": {"name": "foo"}})?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn update(&self, selector: bson::Document, update: bson::Document) -> Result<bson::Document> {
        self.update_with_opts(selector, update, None)
    }

    /// Finds docs the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
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
    /// let count = collection.count(None)?;
    /// assert_eq!(2, count);
    ///
    /// let maybe: Result<Vec<bson::Document>> = collection.find(doc!{"name": "foo"}).collect();
    ///
    /// assert!(maybe.is_ok());
    /// let records = maybe.unwrap();
    ///
    /// assert_eq!(1, records.len());
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn find(&self, filter: bson::Document) -> Self::Cursor {
        self.find_with_opts(filter, None)
    }

    /// Finds docs the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
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
    /// let count = collection.count(None)?;
    /// assert_eq!(2, count);
    ///
    /// let maybe: Result<Vec<bson::Document>> = collection.find(doc!{"name": "foo"}).collect();
    ///
    /// assert!(maybe.is_ok());
    /// let records = maybe.unwrap();
    ///
    /// assert_eq!(1, records.len());
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn find_with_opts(&self, filter: bson::Document, opts: Option<FindAndModify>) -> Self::Cursor {
        let bson_filter = Bsonc::from_document(&filter).expect("should be valid");

        let op = opts.unwrap_or_default();
        let bsonc_opts = op.fields_bsonc().unwrap_or_default();

        let ptr = unsafe {
            bindings::mongoc_collection_find_with_opts(
                self.inner,
                bson_filter.as_ptr(),
                bsonc_opts.as_ptr(),
                ReadPrefsc::default().as_ptr(),
            )
        };

        Cursorc::from_ptr(ptr)
    }

    /// Finds docs the number of documents in a collection.
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// collection.insert_many(vec![
    ///     doc!{"num": 1},
    ///     doc!{"num": 2},
    ///     doc!{"num": 3},
    ///     doc!{"num": 4},
    /// ])?;
    ///
    /// let maybe: Result<Vec<bson::Document>> = collection.aggregate(doc!{"pipeline": [
    /// {
    ///     "$group": {
    ///         "_id": null,
    ///         "total": { "$sum": "$num" }
    ///     }
    /// }
    /// ]}).collect();
    ///
    /// assert!(maybe.is_ok());
    /// let records = maybe.unwrap();
    /// let val = dbg!(records)[0].get_i32("total");
    ///
    /// assert!(val.is_ok());
    /// assert_eq!(10, val.unwrap());
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn aggregate(&self, pipeline: bson::Document) -> Self::Cursor {
        self.aggregate_with_opts(pipeline, None)
    }

    /// Drops collection
    ///
    /// TODO: Add docs
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// collection.drop_collection();
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn drop_collection(&self) -> Result<bool> {
        let mut error = BsoncError::empty();
        let success = unsafe { bindings::mongoc_collection_drop(self.inner, error.as_mut_ptr()) };

        if error.is_empty() {
            Ok(success)
        } else {
            Err(error.into())
        }
    }

    /// Counts the number of documents in a collection.
    ///
    /// From MongoDB Docs
    /// filter: A bson::Document containing the filter.
    /// opts: A bson::Document, None to ignore.
    /// read_prefs: A mongoc_read_prefs_t or None.
    /// opts may be None or a BSON document with additional command options:
    ///  readConcern: Construct a mongoc_read_concern_t and use mongoc_read_concern_append() to add
    ///  the read concern to opts. See the example code for mongoc_client_read_command_with_opts().
    ///  Read concern requires MongoDB 3.2 or later, otherwise an error is returned.
    ///
    ///  sessionId: First, construct a mongoc_client_session_t with mongoc_client_start_session().
    ///  You can begin a transaction with mongoc_client_session_start_transaction(), optionally
    ///  with a mongoc_transaction_opt_t that overrides the options inherited from collection, and
    ///  use mongoc_client_session_append() to add the session to opts. See the example code for
    ///  mongoc_client_session_t.  collation: Configure textual comparisons. See Setting Collation
    ///  Order, and the MongoDB Manual entry on Collation. Collation requires MongoDB 3.2 or later,
    ///  otherwise an error is returned.
    ///
    ///  serverId: To target a specific server, include an int32 “serverId” field. Obtain the id by
    ///  calling mongoc_client_select_server(), then mongoc_server_description_id() on its return
    ///  value.  skip: An int specifying how many documents matching the query should be skipped
    ///  before counting.  limit: An int specifying the maximum number of documents to count.
    ///
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    ///
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let count = collection.count_with_opts(None, Some(Count{ skip: 0, limit: 10 }))?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn count_with_opts(&self, filter: Option<bson::Document>, opts: Option<Count>) -> Result<i64> {
        let bsonc_filter =
            filter.map_or_else(|| Ok(Bsonc::empty()), |d| Bsonc::from_document(&d))?;

        let bsonc_opts = opts.map_or_else(|| Ok(Bsonc::empty()), |d| d.into_mongoc())?;

        let reply = Bsonc::empty();
        let mut error = BsoncError::empty();
        let count = unsafe {
            bindings::mongoc_collection_count_documents(
                self.inner,
                bsonc_filter.as_ptr(),
                bsonc_opts.as_ptr(),
                ptr::null_mut(), // ReadPrefs Use what is defined in the connection
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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// let result = collection.insert_one_with_opts(doc, None)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn insert_one_with_opts(
        &self,
        doc: bson::Document,
        opts: Option<Insert>,
    ) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();

        let bsonc_opts = opts.map_or_else(|| Ok(Bsonc::empty()), Insert::into_mongoc)?;

        let success = unsafe {
            bindings::mongoc_collection_insert_one(
                self.inner,
                Bsonc::from_document(&doc)?.as_ptr(),
                bsonc_opts.as_ptr(),
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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
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
    /// let result = collection.insert_many_with_opts(docs, None)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(4, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn insert_many_with_opts(
        &self,
        docs: Vec<bson::Document>,
        opts: Option<Insert>,
    ) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();

        let bsonc_opts = opts.map_or_else(|| Ok(Bsonc::empty()), Insert::into_mongoc)?;

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
                bsonc_opts.as_ptr(),
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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    /// let selector = doc!{"name": "omg"};
    /// collection.delete_with_opts(selector, None)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(0, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn delete_with_opts(
        &self,
        doc: bson::Document,
        opts: Option<Remove>,
    ) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();

        let bsonc_opts = opts.map_or_else(|| Ok(Bsonc::empty()), Remove::into_mongoc)?;

        let success = unsafe {
            bindings::mongoc_collection_delete_many(
                self.inner,
                Bsonc::from_document(&doc)?.as_ptr(),
                bsonc_opts.as_ptr(),
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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// let doc = doc!{"name": "omg"};
    /// collection.insert_one(doc)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    /// let selector = doc!{"name": "omg"};
    /// collection.update_with_opts(selector, doc!{"$set": {"name": "foo"}}, None)?;
    /// let count = collection.count(None)?;
    /// assert_eq!(1, count);
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn update_with_opts(
        &self,
        selector: bson::Document,
        update: bson::Document,
        opts: Option<Update>,
    ) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();

        let bsonc_opts = opts.map_or_else(|| Ok(Bsonc::empty()), Update::into_mongoc)?;

        let success = unsafe {
            bindings::mongoc_collection_update_many(
                self.inner,
                Bsonc::from_document(&selector)?.as_ptr(),
                Bsonc::from_document(&update)?.as_ptr(),
                bsonc_opts.as_ptr(),
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
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("test");
    /// collection.insert_many(vec![
    ///     doc!{"num": 1},
    ///     doc!{"num": 2},
    ///     doc!{"num": 3},
    ///     doc!{"num": 4},
    /// ])?;
    ///
    /// let maybe: Result<Vec<bson::Document>> = collection.aggregate_with_opts(doc!{"pipeline": [
    /// {
    ///     "$group": {
    ///         "_id": null,
    ///         "total": { "$sum": "$num" }
    ///     }
    /// }
    /// ]}, None).collect();
    ///
    /// assert!(maybe.is_ok());
    /// let records = maybe.unwrap();
    /// let val = dbg!(records)[0].get_i32("total");
    ///
    /// assert!(val.is_ok());
    /// assert_eq!(10, val.unwrap());
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn aggregate_with_opts(&self, pipeline: bson::Document, opts: Option<Aggregate>) -> Cursorc {
        let bsonc_pipeline = Bsonc::from_document(&pipeline).expect("should be valid");

        let agg_opts = opts.unwrap_or_default();
        let bsonc_opts = agg_opts.to_mongoc().expect("should be valid agg opts");

        let read_pref = ReadPrefsc::default();

        let ptr = unsafe {
            bindings::mongoc_collection_aggregate(
                self.inner,
                agg_opts.query_flags.flags(),
                bsonc_pipeline.as_ptr(),
                bsonc_opts.as_ptr(),
                read_pref.as_ptr(),
            )
        };

        Cursorc::from_ptr(ptr)
    }

    /// Watches collection
    ///
    /// TODO: Add docs
    /// NOTE:  Need to setup repl for this to work.  I have not tested with a repl
    /// # Examples
    /// ```no_run
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://repl/?replicaSet=rs0");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    ///
    /// let db = client.default_database();
    /// let collection = db.get_collection("changing");
    /// let stream = collection.watch(None, None)?;
    ///
    /// let docs = vec![
    ///     doc!{"name": "first"},
    ///     doc!{"name": "second"},
    ///     doc!{"name": "third"},
    ///     doc!{"name": "fourth"},
    /// ];
    /// let result = collection.insert_many(docs)?;
    ///
    /// let maybe: Result<Vec<bson::Document>> = stream.collect();
    /// assert!(maybe.is_ok(), "Has error");
    ///
    /// let results = maybe.unwrap();
    /// assert_eq!(4, results.len());
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn watch(
        &self,
        pipeline: Option<bson::Document>,
        _opts: Option<bson::Document>,
    ) -> Result<Self::ChangeStream> {
        let bson_pipeline = pipeline.map_or_else(Bsonc::empty, |o| {
            Bsonc::from_document(&o).expect("should be valid")
        });
        let empty = Bsonc::empty();

        let inner = unsafe {
            bindings::mongoc_collection_watch(
                self.inner,
                bson_pipeline.as_mut_ptr(),
                empty.as_mut_ptr(),
            )
        };

        let change_stream = ChangeStreamc::from_ptr(inner);

        if let Some(error) = change_stream.get_error() {
            return Err(error.into());
        }

        Ok(change_stream)
    }
}

impl Drop for Collectionc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_collection_destroy(self.inner);
            };
            self.inner = ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::env;

    #[test]
    fn test_aggregate_with_options() -> Result<()> {
        env::set_var("MONGODB_URI", "mongodb://standard");
        let builder = Builder::new();
        let pool = builder.random_database_connect()?;
        let client = pool.pop();

        let db = client.default_database();
        let collection = db.get_collection("test");
        collection.insert_many(vec![
            doc! {"num": 1},
            doc! {"num": 2},
            doc! {"num": 3},
            doc! {"num": 4},
        ])?;

        let maybe: Result<Vec<bson::Document>> = super::Collectionc::aggregate_with_opts(
            &collection,
            doc! {"pipeline": [
                {
                    "$group": {
                        "_id": null,
                        "total": { "$sum": "$num" }
                    }
                }
            ]},
            None,
        )
        .collect();

        assert!(maybe.is_ok());
        let records = maybe.unwrap();
        let val = dbg!(records)[0].get_i32("total");

        assert!(val.is_ok());
        assert_eq!(10, val.unwrap());
        db.destroy();
        Ok(())
    }
}
