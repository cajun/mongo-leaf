use crate::{
    bindings,
    error::{MongoError, Result},
};
use std::env;
use std::ffi::CString;
use std::path::Path;

#[derive(Default, Debug)]
pub struct SSLOptions {
    pem_file: Option<String>,
    pem_pwd: Option<String>,
    ca_file: Option<String>,
    ca_dir: Option<String>,
    crl_file: Option<String>,
    weak_cert_validation: Option<bool>,
    allow_invalid_hostname: Option<bool>,
}

impl SSLOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_pem(&mut self, path: String, password: Option<String>) -> &mut Self {
        self.pem_file = Some(path);
        self.pem_pwd = password;
        self
    }

    pub fn set_ca_path(&mut self, path: String, crl_file: Option<String>) -> &mut Self {
        self.pem_file = Some(path);
        self.crl_file = crl_file;
        self
    }

    pub fn set_ca_dir(&mut self, dir: String) -> &mut Self {
        self.ca_dir = Some(dir);
        self
    }

    pub fn set_weak_cert_validation(&mut self, weak: bool) -> &mut Self {
        self.weak_cert_validation = Some(weak);
        self
    }

    pub fn set_allow_invalid_hostname(&mut self, allow: bool) -> &mut Self {
        self.allow_invalid_hostname = Some(allow);
        self
    }

    pub(crate) fn to_mongoc(&self) -> Result<*const bindings::mongoc_ssl_opt_t> {
        let pem_file = env::var("PEM_FILE")
            .map(|v| v)
            .map_err(|_| self.pem_file.as_ref());
        let pem_pwd = env::var("PEM_PWD")
            .map(|v| v)
            .map_err(|_| self.pem_pwd.as_ref());
        let ca_file = env::var("CA_FILE")
            .map(|v| v)
            .map_err(|_| self.ca_file.as_ref());
        let ca_dir = env::var("CA_DIR")
            .map(|v| v)
            .map_err(|_| self.ca_dir.as_ref());
        let crl_file = env::var("CRL_FILE")
            .map(|v| v)
            .map_err(|_| self.crl_file.as_ref());

        let ssl_options = unsafe {
            let ssl_options =
                bindings::mongoc_ssl_opt_get_default() as *mut bindings::mongoc_ssl_opt_t;

            if let Ok(path) = pem_file {
                (*ssl_options).pem_file = CString::new(path)?.as_ptr();
            }
            if let Ok(pwd) = pem_pwd {
                (*ssl_options).pem_pwd = CString::new(pwd)?.as_ptr();
            }
            if let Ok(path) = ca_file {
                (*ssl_options).ca_file = CString::new(path)?.as_ptr();
            }
            if let Ok(path) = ca_dir {
                (*ssl_options).ca_dir = CString::new(path)?.as_ptr();
            }
            if let Ok(path) = crl_file {
                (*ssl_options).crl_file = CString::new(path)?.as_ptr();
            }
            if let Some(weak_cert_validation) = self.weak_cert_validation {
                (*ssl_options).weak_cert_validation = weak_cert_validation;
            }
            if let Some(allow_invalid_hostname) = self.allow_invalid_hostname {
                (*ssl_options).allow_invalid_hostname = allow_invalid_hostname;
            }

            ssl_options
        };

        Ok(ssl_options)
    }
}
