use crate::bindings;
use std::convert::AsRef;
use std::ffi::CStr;

#[derive(Debug)]
pub struct Hostc {
    next: *mut bindings::mongoc_host_list_t,
    pub host: String,
    pub host_and_port: String,
    pub port: u16,
    pub family: i32,
}

pub trait Host {
    fn host(&self) -> &str;
    fn host_and_port(&self) -> &str;
    fn port(&self) -> u16;
    fn family(&self) -> i32;
}

impl<'a> Host for Hostc {
    fn host(&self) -> &str {
        self.host.as_ref()
    }

    fn host_and_port(&self) -> &str {
        self.host_and_port.as_ref()
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn family(&self) -> i32 {
        self.family
    }
}

impl Hostc {
    fn from_ptr(ptr: *const bindings::mongoc_host_list_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let host = unsafe {
                let host = CStr::from_ptr((*ptr).host.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                let host_and_port = CStr::from_ptr((*ptr).host_and_port.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                let port = (*ptr).port;
                let family = (*ptr).family;

                Hostc {
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

    pub fn host_list_from_ptr(ptr: *const bindings::mongoc_host_list_t) -> Vec<Self> {
        let mut next_ptr = ptr;
        let mut hosts = vec![];

        while let Some(h) = Hostc::from_ptr(next_ptr) {
            next_ptr = h.next;
            hosts.push(h);
        }

        hosts
    }
}
