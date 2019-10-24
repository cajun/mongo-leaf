#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bson;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod bsonc;
mod error;
mod uri;

pub use error::{
    BsoncError, BulkOperationError, InvalidParamsError, MongoError, MongoErrorCode,
    MongoErrorDomain,
};

/// Result that's used in all functions that perform operations
/// on the database.
pub type Result<T> = std::result::Result<T, MongoError>;
