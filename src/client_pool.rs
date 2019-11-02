use crate::{
    bindings,
    uri::{Uri, Uric},
};
use std::ptr;

pub struct ClientPoolc {
    uri: Uric,
    inner: *mut bindings::mongoc_client_pool_t,
}

pub trait ClientPool {
    type Inner: ClientPool + Sized;
    type Uri: Uri + Sized;

    fn new(uri: Self::Uri) -> Self::Inner;
    fn destroy(&mut self);
}

impl ClientPool for ClientPoolc {
    type Inner = ClientPoolc;
    type Uri = Uric;

    /// From MongoC documentation:
    ///
    /// mongoc_client_pool_t is the basis for multi-threading in the MongoDB C driver. Since
    /// mongoc_client_t structures are not thread-safe, this structure is used to retrieve a new
    /// mongoc_client_t for a given thread. This structure is thread-safe, except for its
    /// destructor method, mongoc_client_pool_destroy(), which is not thread-safe and must only be
    /// called from one thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let uri = Uric::new("mongodb://localhost/").unwrap();
    /// let pool = ClientPoolc::new(uri);
    /// ```
    fn new(uri: Self::Uri) -> Self::Inner {
        crate::init();
        unsafe {
            let inner = bindings::mongoc_client_pool_new(uri.inner());
            assert!(!inner.is_null());
            ClientPoolc { uri, inner }
        }
    }

    /// From MongoC documentation:
    ///
    /// Release all resources associated with pool, including freeing the structure.
    /// This method is NOT thread safe, and must only be called by one thread.
    /// It should be called once the application is shutting down,
    /// and after all other threads that use clients have been joined.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongoc_to_rs_sys::prelude::*;
    ///
    /// let uri = Uric::new("mongodb://localhost/").unwrap();
    /// let mut pool = ClientPoolc::new(uri);
    /// pool.destroy();
    /// ```
    fn destroy(&mut self) {
        unsafe {
            bindings::mongoc_client_pool_destroy(self.inner);
        }
        self.inner = ptr::null_mut();
    }
}

impl Drop for ClientPoolc {
    fn drop(&mut self) {
        self.destroy();
    }
}
