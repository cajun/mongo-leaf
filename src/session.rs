use crate::{
    bindings,
    bsonc::Bsonc,
    error::{BsoncError, Result},
    transaction_opts::{TransactionOpts, TransactionOptsc},
};
use std::ptr;

#[derive(Debug)]
pub struct Sessionc {
    inner: *mut bindings::mongoc_client_session_t,
}

pub trait Session {
    type TransactionOpts: TransactionOpts + Sized + Default;

    fn start_transaction(&self, opts: Option<Self::TransactionOpts>) -> Result<bool>;
    fn commit(&self) -> Result<bson::Document>;
    fn abort(&self) -> Result<bool>;

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_client_session_t {
        ptr::null_mut()
    }
}

impl Sessionc {
    pub fn from_ptr(inner: *mut bindings::mongoc_client_session_t) -> Self {
        Sessionc { inner }
    }
}

impl Session for Sessionc {
    type TransactionOpts = TransactionOptsc;

    fn start_transaction(&self, opts: Option<Self::TransactionOpts>) -> Result<bool> {
        let mut error = BsoncError::empty();
        let txn_opts = opts.unwrap_or_default();

        let success = unsafe {
            bindings::mongoc_client_session_start_transaction(
                self.inner,
                txn_opts.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if success {
            Ok(success)
        } else {
            Err(error.into())
        }
    }

    fn commit(&self) -> Result<bson::Document> {
        let mut error = BsoncError::empty();
        let reply = Bsonc::empty();

        let success = unsafe {
            bindings::mongoc_client_session_commit_transaction(
                self.inner,
                reply.as_mut_ptr(),
                error.as_mut_ptr(),
            )
        };

        if success {
            reply.as_document()
        } else {
            Err(error.into())
        }
    }

    fn abort(&self) -> Result<bool> {
        let mut error = BsoncError::empty();

        let success = unsafe {
            bindings::mongoc_client_session_abort_transaction(self.inner, error.as_mut_ptr())
        };

        if success {
            Ok(success)
        } else {
            Err(error.into())
        }
    }

    fn as_mut_ptr(&self) -> *mut bindings::mongoc_client_session_t {
        self.inner
    }
}

impl Drop for Sessionc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_client_session_destroy(self.inner);
            }
            self.inner = ptr::null_mut();
        }
    }
}
