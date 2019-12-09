use crate::{
    client_pool::{ClientPool, ClientPoolc},
    error::Result,
    ssl_options::SSLOptions as SSL,
    uri::{Uri, Uric},
};
use rand::prelude::*;
use std::env;

#[derive(Debug)]
pub struct Builder {
    uri: String,
    ssl_options: Option<SSL>,
}

impl Default for Builder {
    fn default() -> Builder {
        let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost/".to_string());
        Builder {
            uri,
            ssl_options: None,
        }
    }
}

impl Builder {
    pub fn new() -> Builder {
        Default::default()
    }
}

pub trait ConstructUri<'a> {
    type SSL: SSLOptions + Connect<'a>;
    fn uri(&mut self, uri_string: impl Into<String>) -> &Self::SSL;
}

pub trait SSLOptions {
    fn pem(&mut self, path: String, password: Option<String>) -> &Self;
    fn ca_file(&mut self, path: String, crl_file: Option<String>) -> &Self;
    fn ca_dir(&mut self, path: String) -> &Self;
    fn weak_cert_validation(&mut self, weak: bool) -> &Self;
    fn allow_invalid_hostname(&mut self, allow: bool) -> &Self;
}

pub trait Connect<'a> {
    type Pool: ClientPool<'a>;

    fn connect(&self) -> Result<Self::Pool>;
    fn random_database_connect(&self) -> Result<Self::Pool>;
}

impl<'a> ConstructUri<'a> for Builder {
    type SSL = Builder;

    fn uri(&mut self, uri_string: impl Into<String>) -> &Self::SSL {
        self.uri = uri_string.into();
        self
    }
}

impl SSLOptions for Builder {
    fn pem(&mut self, path: String, password: Option<String>) -> &Self {
        self.ssl_options
            .as_mut()
            .or_else(Default::default)
            .and_then(|option| {
                option.set_pem(path, password);
                Some(option)
            });

        self
    }
    fn ca_file(&mut self, path: String, crl_file: Option<String>) -> &Self {
        self.ssl_options
            .as_mut()
            .or_else(Default::default)
            .and_then(|option| {
                option.set_ca_path(path, crl_file);
                Some(option)
            });
        self
    }
    fn ca_dir(&mut self, path: String) -> &Self {
        self.ssl_options
            .as_mut()
            .or_else(Default::default)
            .and_then(|option| {
                option.set_ca_dir(path);
                Some(option)
            });
        self
    }
    fn weak_cert_validation(&mut self, weak: bool) -> &Self {
        self.ssl_options
            .as_mut()
            .or_else(Default::default)
            .and_then(|option| {
                option.set_weak_cert_validation(weak);
                Some(option)
            });
        self
    }
    fn allow_invalid_hostname(&mut self, allow: bool) -> &Self {
        self.ssl_options
            .as_mut()
            .or_else(Default::default)
            .and_then(|option| {
                option.set_allow_invalid_hostname(allow);
                Some(option)
            });
        self
    }
}

impl<'a> Connect<'a> for Builder {
    type Pool = ClientPoolc;

    fn connect(&self) -> Result<Self::Pool> {
        let uri = Uric::new(self.uri.clone())?;
        ClientPoolc::new(uri, self.ssl_options.as_ref())
    }

    fn random_database_connect(&self) -> Result<Self::Pool> {
        let uri = Uric::new(dbg!(&self.uri).clone())?;

        let num: i32 = random();

        uri.set_database(format!("mongo_leaf_testing_{:?}", num));

        ClientPoolc::new(uri, self.ssl_options.as_ref())
    }
}
