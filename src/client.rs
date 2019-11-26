use crate::{
    bindings,
    bsonc::Bsonc,
    client_pool::{ClientPool, ClientPoolc},
    collection::{Collection, Collectionc},
    cursor::{Cursor, Cursorc},
    database::{Database, Databasec},
    error::{BsoncError, Result},
    read_prefs::{ReadPrefs, ReadPrefsc},
    session::{Session, Sessionc},
    session_opts::{SessionOpts, SessionOptsc},
};
use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
pub struct Clientc<'a> {
    pub client_pool: &'a ClientPoolc,
    inner: *mut bindings::mongoc_client_t,
}

pub trait Client {
    type Cursor: Cursor;
    type Collection: Collection;
    type ReadPrefs: ReadPrefs + Sized + Default;
    type Database: Database;
    type Session: Session;
    type SessionOpts: SessionOpts + Sized + Default;

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_client_t {
        ptr::null_mut()
    }

    fn destroy(&mut self);
    fn get_database(&self, db_name: impl Into<String>) -> Self::Database;
    fn default_database(&self) -> Self::Database;

    fn command_simple(
        &self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<bson::Document>;

    fn command(
        &self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<Self::Cursor>;

    fn get_collection(
        &self,
        db_name: impl Into<String>,
        collection_name: impl Into<String>,
    ) -> Self::Collection;

    fn start_session(&self, opts: Option<Self::SessionOpts>) -> Result<Self::Session>;
}

impl<'a> Clientc<'a> {
    pub fn new(client_pool: &'a ClientPoolc, inner: *mut bindings::mongoc_client_t) -> Self {
        Clientc { client_pool, inner }
    }
}

impl Client for Clientc<'_> {
    type ReadPrefs = ReadPrefsc;
    type Cursor = Cursorc;
    type Collection = Collectionc;
    type Database = Databasec;
    type Session = Sessionc;
    type SessionOpts = SessionOptsc;

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_client_t {
        self.inner
    }

    /// Executes a simple command against MongoDB
    ///
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard/");
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// let mut client = pool.pop();
    ///
    /// let result = client.command_simple("admin",doc!{"serverStatus":1}, None)?;
    /// assert_eq!("mongod", result.get_str("process")?);
    /// # Ok(())
    /// # }
    /// ```
    fn command_simple(
        &self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let out = Bsonc::from_document(&doc! {})?;
        unsafe {
            let bsonc = Bsonc::from_document(&command)?;
            let readc = read_prefs.unwrap_or_default();

            if let Ok(db_cstring) = CString::new(db_name.into()) {
                bindings::mongoc_client_command_simple(
                    self.inner,
                    db_cstring.as_ptr(),
                    bsonc.as_ptr(),
                    readc.as_ptr(),
                    out.as_mut_ptr(),
                    error.as_mut_ptr(),
                );
            }
        }

        if error.is_empty() {
            out.as_document()
        } else {
            Err(error.into())
        }
    }

    /// Executes a command against MongoDB
    ///
    /// # Examples
    /// ```no_run
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard/");
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// let mut client = pool.pop();
    ///
    /// let cursor = client.command("admin",doc!{"serverStatus":1}, None)?;
    /// let maybe: Result<Vec<bson::Document>> = cursor.collect();
    ///
    /// match maybe {
    ///   Ok(result) => {
    ///     assert_eq!(1, result.len());
    ///     let maybe_stat = result.first();
    ///     assert!(maybe_stat.is_some());
    ///     let stat = maybe_stat.unwrap();
    ///     assert_eq!("mongod",stat.get_str("process")?);
    ///   },
    ///   Err(e) => assert!(false, "Should have a result {:?}", e)
    /// };
    /// # Ok(())
    /// # }
    /// ```
    fn command(
        &self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<Self::Cursor> {
        let bsonc = Bsonc::from_document(&command)?;
        let readc = read_prefs.unwrap_or_default();

        let fields = Bsonc::from_document(&doc! {"fake": 1})?;

        CString::new(db_name.into())
            .map_err(|err| err.into())
            .map(|db_cstring| {
                let ptr = unsafe {
                    bindings::mongoc_client_command(
                        self.inner,
                        db_cstring.as_ptr(),
                        0, // Flags unused
                        0, // Skip unused
                        0, // limit unused
                        0, // Batch Size unused
                        bsonc.as_ptr(),
                        fields.as_ptr(), // Fields unused
                        readc.as_ptr(),
                    )
                };

                Cursorc::from_ptr(ptr)
            })
    }

    /// Create/Get collection from database
    ///
    /// # Examples
    /// ```
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard/");
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// let mut client = pool.pop();
    ///
    /// let collection = client.get_collection("test", "get_collection");
    ///
    /// # let db = client.get_database("test");
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn get_collection(
        &self,
        db_name: impl Into<String>,
        collection_name: impl Into<String>,
    ) -> Self::Collection {
        let ptr = unsafe {
            let db_str = CString::new(db_name.into()).expect("Valid database name");
            let coll_str = CString::new(collection_name.into()).expect("Valid collection name");

            bindings::mongoc_client_get_collection(self.inner, db_str.as_ptr(), coll_str.as_ptr())
        };

        Collectionc::from_ptr(ptr)
    }

    fn get_database(&self, db_name: impl Into<String>) -> Self::Database {
        unsafe {
            let db_cstring = CString::new(db_name.into()).expect("Valid database name");
            let ptr = bindings::mongoc_client_get_database(self.inner, db_cstring.as_ptr());

            Databasec::new(ptr)
        }
    }

    fn default_database(&self) -> Self::Database {
        unsafe {
            let ptr = bindings::mongoc_client_get_default_database(self.inner);
            Databasec::new(ptr)
        }
    }

    fn destroy(&mut self) {
        self.inner = ptr::null_mut();
    }

    /// Start a session and try a transaction
    ///
    /// # Examples
    /// ```no_run
    /// #[macro_use]
    /// extern crate bson;
    /// use mongo_leaf::prelude::*;
    /// use std::env;
    ///
    /// # fn main() -> Result<()> {
    /// env::set_var("MONGODB_URI","mongodb://standard/");
    /// let builder = Builder::new();
    /// let pool = builder.random_database_connect()?;
    /// let mut client = pool.pop();
    /// let db = client.default_database();
    /// let session = client.start_session(None)?;
    ///
    /// let reply = session.start_transaction(None)?;
    /// assert!(reply);
    ///
    /// let collection = db.get_collection("with_txn");
    ///
    /// let reply = collection.insert_many(vec![
    ///     doc!{"name": "first"},
    ///     doc!{"name": "second"},
    ///     doc!{"name": "third"},
    ///     doc!{"name": "fourth"},
    /// ])?;
    ///
    /// let count = collection.count(None)?;
    /// assert_eq!(4, count, "In Transaction");
    ///
    /// let reply = session.abort()?;
    /// assert!(reply);
    ///
    /// let count = collection.count(None)?;
    /// assert_eq!(0, count, "Transaction Aborted");
    ///
    /// # db.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn start_session(&self, opts: Option<Self::SessionOpts>) -> Result<Self::Session> {
        let mut error = BsoncError::empty();
        let ptr = unsafe {
            bindings::mongoc_client_start_session(
                self.inner,
                opts.unwrap_or_default().as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if error.is_empty() {
            Ok(Sessionc::from_ptr(ptr))
        } else {
            Err(error.into())
        }
    }
}

impl<'a> Drop for Clientc<'a> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            self.client_pool.push(self);
            self.destroy();
        }
    }
}
