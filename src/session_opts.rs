use crate::bindings;
use std::ptr;

pub struct SessionOptsc {
    inner: *mut bindings::mongoc_session_opt_t,
}

pub trait SessionOpts {
    fn as_mut_ptr(&self) -> *mut bindings::mongoc_session_opt_t;
}

impl SessionOptsc {
    pub fn from_ptr(inner: *mut bindings::mongoc_session_opt_t) -> Self {
        SessionOptsc { inner }
    }
}

impl Default for SessionOptsc {
    fn default() -> SessionOptsc {
        let inner = unsafe { bindings::mongoc_session_opts_new() };
        SessionOptsc { inner }
    }
}

impl SessionOpts for SessionOptsc {
    fn as_mut_ptr(&self) -> *mut bindings::mongoc_session_opt_t {
        self.inner
    }
}

impl Drop for SessionOptsc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                bindings::mongoc_session_opts_destroy(self.inner);
                self.inner = ptr::null_mut();
            }
        }
    }
}
