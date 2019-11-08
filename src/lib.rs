#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bson;

use std::sync::Once;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod bsonc;
pub mod builder;
mod client;
mod client_pool;
mod cursor;
mod database;
mod error;
mod host;
pub mod prelude;
mod read_prefs;
mod uri;

static MONGOC_INIT: Once = Once::new();

/// Init mongo driver, needs to be called once before doing
/// anything else.
fn init() {
    MONGOC_INIT.call_once(|| {
        unsafe {
            // Init mongoc subsystem
            bindings::mongoc_init();

            // Set mongoc log handler
            //bindings::mongoc_log_set_handler(Some(mongoc_log_handler), ptr::null_mut());
        }
    });
}
