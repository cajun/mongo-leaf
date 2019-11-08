use crate::bindings;
use std::ffi::CStr;

struct Databasec {
    db: *mut bindings::mongoc_database_t,
}

pub trait Database {
    fn name(&self) -> String;
}

impl Database for Databasec {
    fn name(&self) -> String {
        unsafe {
            let cstr = CStr::from_ptr(bindings::mongoc_database_get_name(self.db));
            String::from_utf8_lossy(cstr.to_bytes()).to_string()
        }
    }
}
