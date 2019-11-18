use crate::bindings;
use std::ptr;

pub struct TransactionOptsc {
    inner: *mut bindings::mongoc_transaction_opt_t,
}

pub trait TransactionOpts {
    fn as_mut_ptr(&self) -> *mut bindings::mongoc_transaction_opt_t;
}

impl TransactionOpts for TransactionOptsc {
    fn as_mut_ptr(&self) -> *mut bindings::mongoc_transaction_opt_t {
        self.inner
    }
}

impl Default for TransactionOptsc {
    fn default() -> Self {
        let inner = unsafe { bindings::mongoc_transaction_opts_new() };

        TransactionOptsc { inner }
    }
}

impl Drop for TransactionOptsc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_transaction_opts_destroy(self.inner);
                self.inner = ptr::null_mut();
            }
        }
    }
}
