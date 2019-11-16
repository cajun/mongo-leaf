use crate::{
    client_pool::{ClientPool, ClientPoolc},
    error::Result,
    uri::{Uri, Uric},
};
use rand::prelude::*;
use std::env;

#[derive(Debug)]
pub struct Builder {
    uri: String,
}

impl Default for Builder {
    fn default() -> Builder {
        let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost/".to_string());
        Builder { uri }
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

pub trait SSLOptions {}

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

impl SSLOptions for Builder {}

impl<'a> Connect<'a> for Builder {
    type Pool = ClientPoolc;

    fn connect(&self) -> Result<Self::Pool> {
        let uri = Uric::new(self.uri.clone())?;
        Ok(ClientPoolc::new(uri))
    }

    fn random_database_connect(&self) -> Result<Self::Pool> {
        let uri = Uric::new(self.uri.clone())?;

        let num: i32 = random();
        uri.set_database(format!("testing_{:?}", num));

        Ok(ClientPoolc::new(uri))
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        dbg!("Builder start drop");
        dbg!(self);
        dbg!("Builder start done");
    }
}