use crate::bindings;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr;

pub struct Uri {
    inner: *mut bindings::mongoc_uri_t,
}

pub trait Uric {
    fn new<T: Into<Vec<u8>>>(uri_string: T) -> Option<Uri>;
    fn get_database<'a>(&'a self) -> Option<Cow<'a, str>>;
    fn copy(&self) -> Option<Uri>;
    fn destroy(&mut self);
    fn get_auth_mechanism<'a>(&'a self) -> Option<Cow<'a, str>>;
    fn get_auth_source<'a>(&'a self) -> Option<Cow<'a, str>>;
}

impl Uric for Uri {
    fn new<T: Into<Vec<u8>>>(uri_string: T) -> Option<Uri> {
        CString::new(uri_string).ok().and_then(|uri_cstring| {
            let uri = unsafe { bindings::mongoc_uri_new(uri_cstring.as_ptr()) };

            if uri.is_null() {
                None
            } else {
                Some(Uri { inner: uri })
            }
        })
    }

    fn get_database<'a>(&'a self) -> Option<Cow<'a, str>> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_database(self.inner);
            if ptr.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(ptr);
                Some(String::from_utf8_lossy(cstr.to_bytes()))
            }
        }
    }

    fn copy(&self) -> Option<Uri> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_copy(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Uri { inner: ptr })
            }
        }
    }

    fn destroy(&mut self) {
        unsafe {
            bindings::mongoc_uri_destroy(self.inner);
        }
        self.inner = ptr::null_mut();
    }

    fn get_auth_mechanism<'a>(&'a self) -> Option<Cow<'a, str>> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_auth_mechanism(self.inner);
            if ptr.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(ptr);
                Some(String::from_utf8_lossy(cstr.to_bytes()))
            }
        }
    }

    fn get_auth_source<'a>(&'a self) -> Option<Cow<'a, str>> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_auth_source(self.inner);
            if ptr.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(ptr);
                Some(String::from_utf8_lossy(cstr.to_bytes()))
            }
        }
    }
}

impl Drop for Uri {
    fn drop(&mut self) {
        self.destroy();
    }
}

#[cfg(test)]
mod tests {
    use super::{Uri, Uric};
    use std::borrow::Cow;

    #[test]
    fn create_uri() {
        let uri = Uri::new("mongodb://localhost:27017");
        assert!(uri.is_some());
    }

    #[test]
    fn get_database() {
        let uri = Uri::new("mongodb://localhost:27017/some_db").unwrap();
        assert_eq!(uri.get_database(), Some(Cow::Borrowed("some_db")));
    }

    #[test]
    fn copy() {
        let uri = Uri::new("mongodb://localhost:27017/copied").unwrap();
        let copy = uri.copy();
        assert!(copy.is_some());
        assert_eq!(copy.unwrap().get_database(), Some(Cow::Borrowed("copied")));
    }

    #[test]
    #[should_panic]
    fn destroy() {
        let mut uri = Uri::new("mongodb://localhost:27017/to_destroy").unwrap();
        uri.destroy();
        uri.get_database();
    }

    #[test]
    fn get_auth_mechanism() {
        let uri = Uri::new("mongodb://some@localhost:27017/?authMechanism=CoolBeans").unwrap();
        assert_eq!(uri.get_auth_mechanism(), Some(Cow::Borrowed("CoolBeans")));
    }

    #[test]
    fn get_auth_source() {
        let uri = Uri::new("mongodb://localhost:27017/?authSource=other_db").unwrap();
        assert_eq!(uri.get_auth_source(), Some(Cow::Borrowed("other_db")));
    }
}
