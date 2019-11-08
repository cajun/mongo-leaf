use crate::{
    bindings,
    bsonc::Bsonc,
    client_pool::{ClientPool, ClientPoolc},
    cursor::{Cursor, Cursorc},
    error::{BsoncError, Result},
    read_prefs::{ReadPrefs, ReadPrefsc},
};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::ptr;

#[derive(Debug)]
pub struct Clientc<'a> {
    pub client_pool: &'a ClientPoolc,
    inner: *mut bindings::mongoc_client_t,
}

pub trait Client {
    type Cursor: Cursor;

    type ReadPrefs: ReadPrefs + Sized + Default;
    fn inner(&self) -> *mut bindings::mongoc_client_t {
        ptr::null_mut()
    }
    fn destroy(&mut self);
    fn command_simple(
        &mut self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<bson::Document>;

    fn command(
        &mut self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<Self::Cursor>;
}

impl<'a> Clientc<'a> {
    pub fn new(client_pool: &'a ClientPoolc, inner: *mut bindings::mongoc_client_t) -> Self {
        Clientc { client_pool, inner }
    }
}

impl Client for Clientc<'_> {
    type ReadPrefs = ReadPrefsc;
    type Cursor = Cursorc;

    fn inner(&self) -> *mut bindings::mongoc_client_t {
        self.inner
    }

    fn command_simple(
        &mut self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let mut out = Bsonc::from_document(&doc! {})?;
        unsafe {
            let bsonc = Bsonc::from_document(&command)?;
            let readc = read_prefs.unwrap_or_default();

            if let Ok(db_cstring) = CString::new(db_name.into()) {
                bindings::mongoc_client_command_simple(
                    self.inner,
                    db_cstring.as_ptr(),
                    bsonc.inner(),
                    readc.inner(),
                    out.mut_inner(),
                    error.mut_inner(),
                );
            }
        }

        if error.is_empty() {
            out.as_document()
        } else {
            Err(error.into())
        }
    }

    fn command(
        &mut self,
        db_name: impl Into<String>,
        command: bson::Document,
        read_prefs: Option<Self::ReadPrefs>,
    ) -> Result<Self::Cursor> {
        let mut out = Bsonc::from_document(&doc! {})?;
        unsafe {
            let bsonc = Bsonc::from_document(&command)?;
            let readc = read_prefs.unwrap_or_default();

            CString::new(db_name.into())
                .map_err(|err| err.into())
                .map(|db_cstring| {
                    let ptr = bindings::mongoc_client_command(
                        self.inner,
                        db_cstring.as_ptr(),
                        0, // Flags unused
                        0, // Skip unused
                        0, // limit unused
                        0, // Batch Size unused
                        bsonc.inner(),
                        Bsonc::empty().inner(), // Fields unused
                        readc.inner(),
                    );
                    Cursorc::from_ptr(ptr)
                })
        }
    }

    fn destroy(&mut self) {
        self.inner = ptr::null_mut();
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

impl<'a> Clientc<'a> {}
//    /// Borrow a collection
//    pub fn get_collection(
//        &'a self,
//        db: impl Into<String>,
//        collection: impl Into<String>,
//    ) -> Collection {
//        assert!(!self.inner.is_null());
//        let coll = unsafe { self.collection_ptr(db.into(), collection.into()) };
//        Collection::new(collection::CreatedBy::BorrowedClient(self), coll)
//    }
//
//    /// Take a collection, client is owned by the collection so the collection can easily
//    /// be passed around
//    pub fn take_collection(
//        self,
//        db: impl Into<String>,
//        collection: impl Into<String>,
//    ) -> Collection {
//        assert!(!self.inner.is_null());
//        let coll = unsafe { self.collection_ptr(db.into(), collection.into()) };
//        Collection::new(collection::CreatedBy::OwnedClient(self), coll)
//    }
//
//    unsafe fn collection_ptr(
//        &self,
//        db: Vec<u8>,
//        collection: Vec<u8>,
//    ) -> *mut bindings::mongoc_collection_t {
//        let db_cstring = CString::new(db).unwrap();
//        let collection_cstring = CString::new(collection).unwrap();
//        bindings::mongoc_client_get_collection(
//            self.inner,
//            db_cstring.as_ptr(),
//            collection_cstring.as_ptr(),
//        )
//    }
//
//    /// Borrow a database
//    pub fn get_database<S: Into<Vec<u8>>>(&'a self, db: S) -> Database<'a> {
//        assert!(!self.inner.is_null());
//        let coll = unsafe { self.database_ptr(db.into()) };
//        Database::new(database::CreatedBy::BorrowedClient(self), coll)
//    }
//
//    /// Take a database, client is owned by the database so the database can easily
//    /// be passed around
//    pub fn take_database<S: Into<Vec<u8>>>(self, db: S) -> Database<'a> {
//        assert!(!self.inner.is_null());
//        let coll = unsafe { self.database_ptr(db.into()) };
//        Database::new(database::CreatedBy::OwnedClient(self), coll)
//    }
//
//    unsafe fn database_ptr(&self, db: Vec<u8>) -> *mut bindings::mongoc_database_t {
//        let db_cstring = CString::new(db).unwrap();
//        bindings::mongoc_client_get_database(self.inner, db_cstring.as_ptr())
//    }
//
//    /// Queries the server for the current server status, returns a document with this information.
//    pub fn get_server_status(&self, read_prefs: Option<ReadPrefs>) -> Result<Document> {
//        assert!(!self.inner.is_null());
//
//        // Bsonc to store the reply
//        let mut reply = Bsonc::new();
//        // Empty error that might be filled
//        let mut error = BsoncError::empty();
//
//        let success = unsafe {
//            bindings::mongoc_client_get_server_status(
//                self.inner,
//                match read_prefs {
//                    Some(ref prefs) => prefs.mut_inner(),
//                    None => ptr::null_mut(),
//                },
//                reply.mut_inner(),
//                error.mut_inner(),
//            )
//        };
//
//        if success == 1 {
//            match reply.as_document_utf8_lossy() {
//                Ok(document) => return Ok(document),
//                Err(error) => return Err(error.into()),
//            }
//        } else {
//            Err(error.into())
//        }
//    }
//
//    pub fn read_command_with_opts<S: Into<Vec<u8>>>(
//        &self,
//        db: S,
//        command: &Document,
//        read_prefs: Option<&ReadPrefs>,
//        options: Option<&Document>,
//    ) -> Result<Document> {
//        assert!(!self.inner.is_null());
//
//        let db_cstring = CString::new(db)?;
//
//        // Bsonc to store the reply
//        let mut reply = Bsonc::new();
//        // Empty error that might be filled
//        let mut error = BsoncError::empty();
//
//        let success = unsafe {
//            bindings::mongoc_client_read_command_with_opts(
//                self.inner,
//                db_cstring.as_ptr(),
//                Bsonc::from_document(command)?.inner(),
//                match read_prefs {
//                    Some(ref prefs) => prefs.inner(),
//                    None => ptr::null(),
//                },
//                match options {
//                    Some(ref o) => Bsonc::from_document(o)?.inner(),
//                    None => ptr::null(),
//                },
//                reply.mut_inner(),
//                error.mut_inner(),
//            )
//        };
//
//        if success == 1 {
//            match reply.as_document_utf8_lossy() {
//                Ok(document) => return Ok(document),
//                Err(error) => return Err(error.into()),
//            }
//        } else {
//            Err(error.into())
//        }
//    }
