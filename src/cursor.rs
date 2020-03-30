use crate::{
    bindings,
    bsonc::Bsonc,
    error::{BsoncError, Result},
    host::{Host, Hostc},
};

use futures::stream::Stream;
use std::{
    ptr,
    pin::Pin,
    sync::{Arc,Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration
};

#[derive(Debug)]
struct SharedState {
    waker: Option<Waker>
}

#[derive(Debug)]
pub struct Cursorc {
    inner: *mut bindings::mongoc_cursor_t,
    shared_state: Arc<Mutex<SharedState>>
}

pub trait Cursor {
    type Host: Host;
    fn get_hosts(&self) -> Option<Vec<Self::Host>>;
}

impl Cursorc {
    pub fn from_ptr(inner: *mut bindings::mongoc_cursor_t) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState{ waker: None }));

        let thread_shared_state = shared_state.clone();

        let duration = Duration::from_nanos(100);
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        Cursorc { inner, shared_state }
    }

    pub fn get_error(&self) -> Option<BsoncError> {
        assert!(!self.inner.is_null());

        let mut error = BsoncError::empty();

        unsafe {
            bindings::mongoc_cursor_error(self.inner, error.as_mut_ptr());
        }

        if error.is_empty() {
            None
        } else {
            Some(error)
        }
    }
}

impl Cursor for Cursorc {
    type Host = Hostc;

    fn get_hosts(&self) -> Option<Vec<Self::Host>> {
        assert!(!self.inner.is_null());

        unsafe {
            let ptr = ptr::null_mut();
            bindings::mongoc_cursor_get_host(self.inner, ptr);
            if ptr.is_null() {
                None
            } else {
                let hosts = Hostc::host_list_from_ptr(ptr);
                Some(hosts)
            }
        }
    }
}

impl Iterator for Cursorc {
    type Item = Result<bson::Document>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bson_ptr: *const bindings::bson_t = ptr::null_mut();

        let success = unsafe { bindings::mongoc_cursor_next(self.inner, &mut bson_ptr) };

        if let Some(err) = self.get_error() {
            Some(Err(err.into()))
        } else if success {
            let bsonc = Bsonc::from_ptr(bson_ptr);
            Some(bsonc.as_document())
        } else {
            None
        }
    }
}

impl Stream for Cursorc {
    type Item = Result<bson::Document>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {

        let mut bson_ptr: *const bindings::bson_t = ptr::null_mut();

        let success = unsafe { bindings::mongoc_cursor_next(self.inner, &mut bson_ptr) };

        if let Some(err) = self.get_error() {
            Poll::Ready(Some(Err(err.into())))
        } else if success {
            let bsonc = Bsonc::from_ptr(bson_ptr);
            Poll::Ready(Some(bsonc.as_document()))
        } else {
            Poll::Pending
        }

    }
}

impl Drop for Cursorc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                dbg!("cursor drop start");
                bindings::mongoc_cursor_destroy(dbg!(self.inner));
                self.inner = ptr::null_mut();
                dbg!("cursor drop done");
            }
        }
    }
}
