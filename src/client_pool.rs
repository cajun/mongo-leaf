use crate::{
    bindings,
    client::{Client, Clientc},
    error::Result,
    ssl_options::SSLOptions,
    uri::{Uri, Uric},
};
use std::ptr;

#[derive(Eq, Debug)]
pub struct ClientPoolc {
    uri: Uric,
    inner: *mut bindings::mongoc_client_pool_t,
}

unsafe impl Send for ClientPoolc {}
unsafe impl Sync for ClientPoolc {}

pub trait ClientPool<'a> {
    type Pool: ClientPool<'a> + Sized;
    type Uri: Uri + Sized;
    type Client: Client;

    fn destroy(&mut self);
    fn pop(&'a self) -> Self::Client;
    fn push(&self, client: &mut Self::Client);
}

impl ClientPoolc {
    /// From MongoC documentation:
    ///
    /// mongoc_client_pool_t is the basis for multi-threading in the MongoDB C driver. Since
    /// mongoc_client_t structures are not thread-safe, this structure is used to retrieve a new
    /// mongoc_client_t for a given thread. This structure is thread-safe, except for its
    /// destructor method, mongoc_client_pool_destroy(), which is not thread-safe and must only be
    /// called from one thread.
    ///
    /// # Examples
    /// ```
    /// use mongo_leaf::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// # Ok(())
    /// # }
    /// ```
    pub(crate) fn new(uri: Uric, ssl_options: Option<&SSLOptions>) -> Result<Self> {
        crate::init();
        unsafe {
            let inner = bindings::mongoc_client_pool_new(uri.as_mut_ptr());

            if let Some(options) = ssl_options {
                bindings::mongoc_client_pool_set_ssl_opts(inner, options.to_mongoc()?);
            }

            assert!(!inner.is_null());
            Ok(ClientPoolc { uri, inner })
        }
    }
}

impl<'a> ClientPool<'a> for ClientPoolc {
    type Pool = ClientPoolc;
    type Uri = Uric;
    type Client = Clientc<'a>;

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
    /// use mongo_leaf::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let mut pool = builder.connect()?;
    /// pool.destroy();
    /// # Ok(())
    /// # }
    /// ```
    fn destroy(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_client_pool_destroy(self.inner);
            }
            self.inner = ptr::null_mut();
        }
    }

    /// From MongoC documentation:
    ///
    /// Retrieve a mongoc_client_t from the client pool, or create one. The total number of clients
    /// that can be created from this pool is limited by the URI option “maxPoolSize”, default 100.
    /// If this number of clients has been created and all are in use, mongoc_client_pool_pop blocks
    /// until another thread returns a client with mongoc_client_pool_push().
    ///
    /// # Examples
    ///
    /// ```
    /// use mongo_leaf::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// let client = pool.pop();
    /// assert_eq!(*client.client_pool, pool);
    /// # Ok(())
    /// # }
    /// ```
    fn pop(&'a self) -> Self::Client {
        let clientc = unsafe { bindings::mongoc_client_pool_pop(self.inner) };

        Clientc::new(self, clientc)
    }

    /// From MongoC documentation:
    ///
    /// This function returns a mongoc_client_t back to the client pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use mongo_leaf::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let builder = Builder::new();
    /// let pool = builder.connect()?;
    /// let mut client = pool.pop();
    /// assert_eq!(*client.client_pool, pool);
    /// pool.push(&mut client);
    /// # Ok(())
    /// # }
    /// ```
    fn push(&self, client: &mut Self::Client) {
        if !client.as_mut_ptr().is_null() {
            unsafe {
                bindings::mongoc_client_pool_push(self.inner, client.as_mut_ptr());
            }
            client.destroy();
        }
    }
}

impl Drop for ClientPoolc {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl PartialEq for ClientPoolc {
    fn eq(&self, other: &Self) -> bool {
        self.uri.eq(&other.uri)
    }
}
