use crate::{
    client_pool::{ClientPool, ClientPoolc},
    error::Result,
    uri::{Uri, Uric},
};

struct Builder {
    uri: String,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            uri: "".to_string(),
        }
    }
}

trait ConstructUri<'a> {
    type SSL: SSLOptions + Connect<'a>;
    fn uri(mut self, uri: impl Into<String>) -> Self::SSL;
}

trait SSLOptions {}

trait Connect<'a> {
    type Pool: ClientPool<'a>;

    fn connect(self) -> Result<Self::Pool>;
}

impl<'a> ConstructUri<'a> for Builder {
    type SSL = Builder;

    fn uri(mut self, uri: impl Into<String>) -> Self::SSL {
        self.uri = uri.into();
        self
    }
}

impl SSLOptions for Builder {}

impl<'a> Connect<'a> for Builder {
    type Pool = ClientPoolc;

    fn connect(self) -> Result<Self::Pool> {
        let uri = Uric::new(self.uri)?;
        Ok(ClientPoolc::new(uri))
    }
}
