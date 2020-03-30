#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bson;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate futures;

extern crate mongo_c_sys;

use std::sync::Once;

use mongo_c_sys::bindings as bindings;

mod bsonc;
pub mod builder;
mod change_stream;
mod client;
mod client_pool;
mod collection;
mod cursor;
mod database;
mod error;
mod flags;
mod host;
mod options;
pub mod prelude;
mod read_concern;
mod read_prefs;
mod session;
mod session_opts;
mod ssl_options;
mod transaction_opts;
mod uri;
mod write_concern;
//mod write_opts;

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
