use crate::{bindings, bsonc::Bsonc, error::BsoncError, Result};
use bson::Document;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Debug)]
pub struct Uri {
    inner: *mut bindings::mongoc_uri_t,
}

#[derive(Debug)]
pub struct Host<'a> {
    next: *mut bindings::mongoc_host_list_t,
    pub host: Cow<'a, str>,
    pub host_and_port: Cow<'a, str>,
    pub port: u16,
    pub family: i32,
}

impl<'a> Host<'a> {
    fn from_ptr(ptr: *const bindings::mongoc_host_list_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let host = unsafe {
                let host = CStr::from_ptr((*ptr).host.as_ptr()).to_string_lossy();
                let host_and_port = CStr::from_ptr((*ptr).host_and_port.as_ptr()).to_string_lossy();
                let port = (*ptr).port;
                let family = (*ptr).family;

                Host {
                    next: (*ptr).next,
                    host,
                    host_and_port,
                    port,
                    family,
                }
            };

            Some(host)
        }
    }

    fn host_list_from_ptr(ptr: *const bindings::mongoc_host_list_t) -> Vec<Self> {
        let mut next_ptr = ptr;
        let mut hosts = vec![];

        while let Some(h) = Host::from_ptr(next_ptr) {
            next_ptr = h.next;
            hosts.push(h);
        }

        hosts
    }
}

pub trait Uric {
    fn new<T: Into<Vec<u8>>>(uri_string: T) -> Option<Uri>;
    fn new_with_result<T: Into<Vec<u8>>>(uri_string: T) -> Result<Uri>;
    fn get_database<'a>(&'a self) -> Option<Cow<'a, str>>;
    fn copy(&self) -> Option<Uri>;
    fn destroy(&mut self);
    fn get_auth_mechanism<'a>(&'a self) -> Option<Cow<'a, str>>;
    fn get_auth_source<'a>(&'a self) -> Option<Cow<'a, str>>;
    fn get_compressors<'a>(&'a self) -> Option<bson::Document>;
    fn get_hosts<'a, 'b>(&'a self) -> Option<Vec<Host<'b>>>;
}

impl Uric for Uri {
    /// Creates a new Uri String
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let valid_uris = vec![
    ///   "mongodb://localhost/",
    ///   "mongodb://localhost/?replicaSet=myreplset",
    ///   "mongodb://myuser:mypass@localhost/",
    ///   "mongodb://kerberosuser%40EXAMPLE.COM@example.com/?authMechanism=GSSAPI",
    ///   "mongodb://[::1]:27017/",
    ///   "mongodb://10.0.0.1:27017,10.0.0.1:27018,[::1]:27019/?ssl=true",
    ///   "mongodb://%2Ftmp%2Fmongodb-27017.sock",
    ///   "mongodb://user:pass@%2Ftmp%2Fmongodb-27017.sock",
    ///   "mongodb://localhost,[::1]/mydb?authSource=mydb"
    /// ];
    ///
    /// valid_uris.iter().for_each(|valid| {
    ///   let uri = Uri::new(valid.to_string());
    ///   assert!(uri.is_some());
    /// });
    /// ```
    ///
    /// Invalid Uri's
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let uri = Uri::new("failme://localhost");
    /// assert!(uri.is_none());
    /// ```
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

    /// Creates a new Uri String with Result
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let valid_uris = vec![
    ///   "mongodb://localhost/",
    ///   "mongodb://localhost/?replicaSet=myreplset",
    ///   "mongodb://myuser:mypass@localhost/",
    ///   "mongodb://kerberosuser%40EXAMPLE.COM@example.com/?authMechanism=GSSAPI",
    ///   "mongodb://[::1]:27017/",
    ///   "mongodb://10.0.0.1:27017,10.0.0.1:27018,[::1]:27019/?ssl=true",
    ///   "mongodb://%2Ftmp%2Fmongodb-27017.sock",
    ///   "mongodb://user:pass@%2Ftmp%2Fmongodb-27017.sock",
    ///   "mongodb://localhost,[::1]/mydb?authSource=mydb"
    /// ];
    ///
    /// valid_uris.iter().for_each(|valid| {
    ///   let uri = Uri::new_with_result(valid.to_string());
    ///   assert!(uri.is_ok());
    /// });
    /// ```
    ///
    /// Invalid Uri's
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let uri = Uri::new_with_result("failme://localhost");
    /// assert!(uri.is_err(), "{:?}", uri);
    /// ```
    fn new_with_result<T: Into<Vec<u8>>>(uri_string: T) -> Result<Uri> {
        let uri_cstring = CString::new(uri_string)?;

        let mut error = BsoncError::empty();

        let uri =
            unsafe { bindings::mongoc_uri_new_with_error(uri_cstring.as_ptr(), error.mut_inner()) };

        if uri.is_null() {
            Err(error.into())
        } else {
            Ok(Uri { inner: uri })
        }
    }

    /// Fetches the database portion of an URI if provided. This is the portion after the / but
    /// before the ?.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uri::new("mongodb://localhost:27017/some_db").unwrap();
    /// assert_eq!(uri.get_database(), Some(Cow::Borrowed("some_db")));
    /// ```
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

    /// Fetches a linked list of hosts that were defined in the URI (the comma-separated host
    /// section).
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uri::new("mongodb://localhost:27017/some_db").unwrap();
    ///
    /// let maybe_hosts = uri.get_hosts();
    /// assert!(maybe_hosts.is_some());
    /// let hosts = maybe_hosts.unwrap();
    /// assert_eq!(hosts.len(), 1);
    /// let host = hosts.first().unwrap();
    /// assert_eq!(host.host, Cow::Borrowed("localhost"));
    /// assert_eq!(host.port, 27017, "Err {:?}", host);
    /// assert_eq!(host.host_and_port, Cow::Borrowed("localhost:27017"), "Err {:?}", host);
    /// assert_eq!(host.family, 0, "Err {:?}", host);
    /// ```
    ///
    /// # Multiple Hosts
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uri::new("mongodb://snoopy:5544,woodstock:4455/").unwrap();
    ///
    /// let maybe_hosts = uri.get_hosts();
    /// assert!(maybe_hosts.is_some());
    /// let hosts = maybe_hosts.unwrap();
    /// assert_eq!(hosts.len(), 2);
    /// let snoopy = hosts.first().unwrap();
    /// assert_eq!(snoopy.host_and_port, Cow::Borrowed("snoopy:5544"));
    /// let woodstock = hosts.last().unwrap();
    /// assert_eq!(woodstock.host_and_port, Cow::Borrowed("woodstock:4455"));
    /// ```
    fn get_hosts<'a, 'b>(&'a self) -> Option<Vec<Host<'b>>> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_hosts(self.inner);
            if ptr.is_null() {
                None
            } else {
                let hosts = Host::host_list_from_ptr(ptr);
                Some(hosts)
            }
        }
    }

    /// Copies the entire contents of a URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uri::new("mongodb://localhost:27017/copied").unwrap();
    /// let copy = uri.copy();
    /// assert!(copy.is_some());
    /// assert_eq!(copy.unwrap().get_database(), Some(Cow::Borrowed("copied")));
    /// ```
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

    /// Frees all resources associated with a uri.  NOTE:  Other functions off of Uri will panic
    /// after the uri has been destroyed
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let mut uri = Uri::new("mongodb://localhost:27017/to_destroy").unwrap();
    /// uri.destroy();
    /// uri.get_database();
    /// ```
    fn destroy(&mut self) {
        unsafe {
            bindings::mongoc_uri_destroy(self.inner);
        }
        self.inner = ptr::null_mut();
    }

    /// Fetches the authMechanism parameter to an URI if provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate bson;
    /// # fn main() {
    /// use mongoc_to_rs_sys::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uri::new("mongodb://some@localhost:27017/?authMechanism=CoolBeans").unwrap();
    /// assert_eq!(uri.get_auth_mechanism(), Some(Cow::Borrowed("CoolBeans")));
    /// # }
    /// ```
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

    /// Fetches the authSource parameter of an URI if provided.
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate bson;
    /// # fn main() {
    /// use mongoc_to_rs_sys::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uri::new("mongodb://localhost:27017/?authSource=other_db").unwrap();
    /// assert_eq!(uri.get_auth_source(), Some(Cow::Borrowed("other_db")));
    /// # }
    /// ```
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

    /// Returns a bson document with the enabled compressors as the keys
    /// if uri has compressors provided, otherwise None.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate bson;
    /// # fn main() {
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let uri = Uri::new("mongodb://localhost:27017/?compressors=zlib,zstd").unwrap();
    /// assert_eq!(
    ///     uri.get_compressors(),
    ///     Some(doc! {"zlib": "yes", "zstd": "yes"})
    /// );
    /// # }
    /// ```
    fn get_compressors<'a>(&'a self) -> Option<bson::Document> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_compressors(self.inner);
            if ptr.is_null() {
                None
            } else {
                let bson = Bsonc::from_ptr(ptr);
                bson.as_document().ok()
            }
        }
    }
}

impl Drop for Uri {
    fn drop(&mut self) {
        self.destroy();
    }
}
