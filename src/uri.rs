use crate::{
    bindings,
    bsonc::Bsonc,
    error::{BsoncError, Result},
    host::{Host, Hostc},
};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Eq, Debug)]
pub struct Uric {
    inner: *mut bindings::mongoc_uri_t,
}

pub trait Uri {
    type Inner: Uri + Sized;
    type Host: Host;

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_uri_t {
        ptr::null_mut()
    }
    fn new(uri_string: impl Into<String>) -> Result<Self::Inner>;
    fn get_database(&self) -> Option<Cow<str>>;
    fn copy(&self) -> Option<Self::Inner>;
    fn destroy(&mut self);
    fn get_auth_mechanism(&self) -> Option<Cow<str>>;
    fn get_auth_source(&self) -> Option<Cow<str>>;
    fn get_compressors(&self) -> Option<bson::Document>;
    fn get_hosts(&self) -> Option<Vec<Self::Host>>;
    fn as_str(&self) -> Cow<str>;
    fn set_database(&self, db_name: impl Into<String>) -> bool;
}

impl Uri for Uric {
    type Inner = Uric;
    type Host = Hostc;

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_uri_t {
        self.inner
    }

    /// Creates a new Uri String with Result
    ///
    /// # Examples
    ///
    /// ```
    /// use mongo_leaf::prelude::*;
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
    /// ];
    ///
    /// valid_uris.iter().for_each(|valid| {
    ///   let uri = Uric::new(valid.to_string());
    ///   assert!(uri.is_ok(), "Not ok for {}", valid.to_string());
    /// });
    /// ```
    ///
    /// Invalid Uri's
    /// ```
    /// use mongo_leaf::prelude::*;
    ///
    /// let uri = Uric::new("failme://localhost");
    /// assert!(uri.is_err(), "{:?}", uri);
    /// ```
    fn new(uri_string: impl Into<String>) -> Result<Self::Inner> {
        let uri_cstring = CString::new(uri_string.into())?;

        let mut error = BsoncError::empty();

        let uri = unsafe {
            bindings::mongoc_uri_new_with_error(uri_cstring.as_ptr(), error.as_mut_ptr())
        };

        if uri.is_null() {
            Err(error.into())
        } else {
            Ok(Uric { inner: uri })
        }
    }

    /// Fetches the database portion of an URI if provided. This is the portion after the / but
    /// before the ?.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongo_leaf::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uric::new("mongodb://localhost:27017/some_db").unwrap();
    /// assert_eq!(uri.get_database(), Some(Cow::Borrowed("some_db")));
    /// ```
    fn get_database(&self) -> Option<Cow<str>> {
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
    /// use mongo_leaf::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uric::new("mongodb://localhost:27017/some_db").unwrap();
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
    /// use mongo_leaf::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uric::new("mongodb://snoopy:5544,woodstock:4455/").unwrap();
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
    fn get_hosts(&self) -> Option<Vec<Self::Host>> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_hosts(self.inner);
            if ptr.is_null() {
                None
            } else {
                let hosts = Hostc::host_list_from_ptr(ptr);
                Some(hosts)
            }
        }
    }

    /// Copies the entire contents of a URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongo_leaf::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uric::new("mongodb://localhost:27017/copied").unwrap();
    /// let copy = uri.copy();
    /// assert!(copy.is_some());
    /// assert_eq!(copy.unwrap().get_database(), Some(Cow::Borrowed("copied")));
    /// ```
    fn copy(&self) -> Option<Self::Inner> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_copy(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Uric { inner: ptr })
            }
        }
    }

    /// Frees all resources associated with a uri.  NOTE:  Other functions off of Uri will panic
    /// after the uri has been destroyed
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use mongo_leaf::prelude::*;
    ///
    /// let mut uri = Uric::new("mongodb://localhost:27017/to_destroy").unwrap();
    /// uri.destroy();
    /// uri.get_database();
    /// ```
    fn destroy(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_uri_destroy(self.inner);
            }
            self.inner = ptr::null_mut();
        }
    }

    /// Fetches the authMechanism parameter to an URI if provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate bson;
    /// # fn main() {
    /// use mongo_leaf::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uric::new("mongodb://some@localhost:27017/?authMechanism=CoolBeans").unwrap();
    /// assert_eq!(uri.get_auth_mechanism(), Some(Cow::Borrowed("CoolBeans")));
    /// # }
    /// ```
    fn get_auth_mechanism(&self) -> Option<Cow<str>> {
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
    /// use mongo_leaf::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let uri = Uric::new("mongodb://some:user@localhost:27017/?authSource=other_db").unwrap();
    /// assert_eq!(uri.get_auth_source(), Some(Cow::Borrowed("other_db")));
    /// # }
    /// ```
    fn get_auth_source(&self) -> Option<Cow<str>> {
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
    /// use mongo_leaf::prelude::*;
    ///
    /// let uri = Uric::new("mongodb://localhost:27017/?compressors=zlib").unwrap();
    /// assert_eq!(
    ///     uri.get_compressors(),
    ///     Some(doc! {"zlib": "yes"})
    /// );
    /// # }
    /// ```
    fn get_compressors(&self) -> Option<bson::Document> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = bindings::mongoc_uri_get_compressors(self.inner) as *mut bindings::bson_t;
            if ptr.is_null() {
                None
            } else {
                let bson = Bsonc::from_ptr(ptr);
                bson.as_document().ok()
            }
        }
    }

    fn set_database(&self, db_name: impl Into<String>) -> bool {
        let db_cstring = CString::new(db_name.into()).expect("Valid database name");
        unsafe { bindings::mongoc_uri_set_database(self.inner, db_cstring.as_ptr()) }
    }

    fn as_str(&self) -> Cow<str> {
        assert!(!self.inner.is_null());
        unsafe {
            let cstr = CStr::from_ptr(bindings::mongoc_uri_get_string(self.inner));
            String::from_utf8_lossy(cstr.to_bytes())
        }
    }
}

unsafe impl Send for Uric {}
unsafe impl Sync for Uric {}

impl Drop for Uric {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Clone for Uric {
    fn clone(&self) -> Uric {
        Uric::new(self.as_str().into_owned()).unwrap()
    }
}

impl PartialEq for Uric {
    fn eq(&self, other: &Uric) -> bool {
        self.as_str() == other.as_str()
    }
}
