use crate::bindings;
use std::borrow::Cow;
use std::error;
use std::ffi::CStr;
use std::fmt;

use bson::{DecoderError, Document, EncoderError, ValueAccessError};
use std::ffi::NulError;

/// Wrapper for all errors that can occur in the driver.
pub enum MongoError {
    /// Error in the underlying C driver.
    Bsonc(BsoncError),
    /// Error decoding Bson.
    Decoder(DecoderError),
    /// Error encoding Bson.
    Encoder(EncoderError),
    /// Error accessing a value on a Bson document.
    ValueAccessError(ValueAccessError),
    /// Invalid params error that can be reported by the underlying C driver.
    InvalidParams(InvalidParamsError),
    // from CString::new(db)
    Nul(NulError),
}

impl fmt::Display for MongoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MongoError::Bsonc(ref err) => write!(f, "{}", err),
            MongoError::Encoder(ref err) => write!(f, "{}", err),
            MongoError::Decoder(ref err) => write!(f, "{}", err),
            MongoError::ValueAccessError(ref err) => write!(f, "{}", err),
            MongoError::InvalidParams(ref err) => write!(f, "{}", err),
            MongoError::Nul(ref err) => write!(f, "{}", err),
        }
    }
}

impl fmt::Debug for MongoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MongoError::Bsonc(ref err) => write!(f, "MongoError ({:?})", err),
            MongoError::Decoder(ref err) => write!(f, "MongoError ({:?})", err),
            MongoError::Encoder(ref err) => write!(f, "MongoError ({:?})", err),
            MongoError::ValueAccessError(ref err) => write!(f, "MongoError ({:?})", err),
            MongoError::InvalidParams(ref err) => write!(f, "MongoError ({:?})", err),
            MongoError::Nul(ref err) => write!(f, "MongoError ({:?})", err),
        }
    }
}

impl error::Error for MongoError {
    fn description(&self) -> &str {
        match *self {
            MongoError::Bsonc(ref err) => err.description(),
            MongoError::Decoder(ref err) => err.description(),
            MongoError::Encoder(ref err) => err.description(),
            MongoError::ValueAccessError(ref err) => err.description(),
            MongoError::InvalidParams(ref err) => err.description(),
            MongoError::Nul(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MongoError::Bsonc(ref err) => Some(err),
            MongoError::Decoder(ref err) => Some(err),
            MongoError::Encoder(ref err) => Some(err),
            MongoError::ValueAccessError(ref err) => Some(err),
            MongoError::InvalidParams(ref err) => Some(err),
            MongoError::Nul(ref err) => Some(err),
        }
    }
}

impl From<DecoderError> for MongoError {
    fn from(error: DecoderError) -> MongoError {
        MongoError::Decoder(error)
    }
}

impl From<EncoderError> for MongoError {
    fn from(error: EncoderError) -> MongoError {
        MongoError::Encoder(error)
    }
}

impl From<ValueAccessError> for MongoError {
    fn from(error: ValueAccessError) -> MongoError {
        MongoError::ValueAccessError(error)
    }
}

impl From<NulError> for MongoError {
    fn from(error: NulError) -> MongoError {
        MongoError::Nul(error)
    }
}

/// Error in the underlying C driver.
pub struct BsoncError {
    inner: bindings::bson_error_t,
}

/// MongoDB error domain.
#[derive(Debug, PartialEq)]
pub enum MongoErrorDomain {
    Blank,
    Client,
    Stream,
    Protocol,
    Cursor,
    Query,
    Insert,
    Sasl,
    Bson,
    Matcher,
    Namespace,
    Command,
    Collection,
    Gridfs,
    Scram,
    Unknown,
}

/// MongoDB error code.
#[derive(Debug, PartialEq)]
pub enum MongoErrorCode {
    Blank,
    StreamInvalidType,
    StreamInvalidState,
    StreamNameResolution,
    StreamSocket,
    StreamConnect,
    StreamNotEstablished,
    ClientNotReady,
    ClientTooBig,
    ClientTooSmall,
    ClientGetnonce,
    ClientAuthenticate,
    ClientNoAcceptablePeer,
    ClientInExhaust,
    ProtocolInvalidReply,
    ProtocolBadWireVersion,
    CursorInvalidCursor,
    QueryFailure,
    BsonInvalid,
    MatcherInvalid,
    NamespaceInvalid,
    NamespaceInvalidFilterType,
    CommandInvalidArg,
    CollectionInsertFailed,
    CollectionUpdateFailed,
    CollectionDeleteFailed,
    CollectionDoesNotExist,
    GridfsInvalidFilename,
    ScramNotDone,
    ScramProtocolError,
    QueryCommandNotFound,
    QueryNotTailable,
    WriteConcernError,
    DuplicateKey,
    Unknown(u32),
}

impl BsoncError {
    pub fn empty() -> BsoncError {
        BsoncError {
            inner: bindings::bson_error_t {
                domain: 0,
                code: 0,
                message: [0; 504],
            },
        }
    }

    /// Wether the error has content.
    pub fn is_empty(&self) -> bool {
        self.inner.domain == 0 && self.inner.code == 0
    }

    /// The error's domain.
    pub fn domain(&self) -> MongoErrorDomain {
        match self.inner.domain {
            0 => MongoErrorDomain::Blank,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_CLIENT => MongoErrorDomain::Client,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_STREAM => MongoErrorDomain::Stream,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_PROTOCOL => MongoErrorDomain::Protocol,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_CURSOR => MongoErrorDomain::Cursor,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_QUERY => MongoErrorDomain::Query,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_INSERT => MongoErrorDomain::Insert,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_SASL => MongoErrorDomain::Sasl,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_BSON => MongoErrorDomain::Bson,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_MATCHER => MongoErrorDomain::Matcher,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_NAMESPACE => MongoErrorDomain::Namespace,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_COMMAND => MongoErrorDomain::Command,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_COLLECTION => MongoErrorDomain::Collection,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_GRIDFS => MongoErrorDomain::Gridfs,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_SCRAM => MongoErrorDomain::Scram,
            _ => MongoErrorDomain::Unknown,
        }
    }

    /// The error's code.
    pub fn code(&self) -> MongoErrorCode {
        match self.inner.code {
            0 => MongoErrorCode::Blank,
            bindings::mongoc_error_code_t_MONGOC_ERROR_STREAM_INVALID_TYPE => {
                MongoErrorCode::StreamInvalidType
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_STREAM_INVALID_STATE => {
                MongoErrorCode::StreamInvalidState
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_STREAM_NAME_RESOLUTION => {
                MongoErrorCode::StreamNameResolution
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_STREAM_SOCKET => {
                MongoErrorCode::StreamSocket
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_STREAM_CONNECT => {
                MongoErrorCode::StreamConnect
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_STREAM_NOT_ESTABLISHED => {
                MongoErrorCode::StreamNotEstablished
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_NOT_READY => {
                MongoErrorCode::ClientNotReady
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_TOO_BIG => {
                MongoErrorCode::ClientTooBig
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_TOO_SMALL => {
                MongoErrorCode::ClientTooSmall
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_GETNONCE => {
                MongoErrorCode::ClientGetnonce
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_AUTHENTICATE => {
                MongoErrorCode::ClientAuthenticate
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_NO_ACCEPTABLE_PEER => {
                MongoErrorCode::ClientNoAcceptablePeer
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_IN_EXHAUST => {
                MongoErrorCode::ClientInExhaust
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_PROTOCOL_INVALID_REPLY => {
                MongoErrorCode::ProtocolInvalidReply
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_PROTOCOL_BAD_WIRE_VERSION => {
                MongoErrorCode::ProtocolBadWireVersion
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CURSOR_INVALID_CURSOR => {
                MongoErrorCode::CursorInvalidCursor
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_QUERY_FAILURE => {
                MongoErrorCode::QueryFailure
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_BSON_INVALID => MongoErrorCode::BsonInvalid,
            bindings::mongoc_error_code_t_MONGOC_ERROR_MATCHER_INVALID => {
                MongoErrorCode::MatcherInvalid
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_NAMESPACE_INVALID => {
                MongoErrorCode::NamespaceInvalid
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_NAMESPACE_INVALID_FILTER_TYPE => {
                MongoErrorCode::NamespaceInvalidFilterType
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_COMMAND_INVALID_ARG => {
                MongoErrorCode::CommandInvalidArg
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_COLLECTION_INSERT_FAILED => {
                MongoErrorCode::CollectionInsertFailed
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_COLLECTION_UPDATE_FAILED => {
                MongoErrorCode::CollectionUpdateFailed
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_COLLECTION_DELETE_FAILED => {
                MongoErrorCode::CollectionDeleteFailed
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_COLLECTION_DOES_NOT_EXIST => {
                MongoErrorCode::CollectionDoesNotExist
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_GRIDFS_INVALID_FILENAME => {
                MongoErrorCode::GridfsInvalidFilename
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_SCRAM_NOT_DONE => {
                MongoErrorCode::ScramNotDone
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_SCRAM_PROTOCOL_ERROR => {
                MongoErrorCode::ScramProtocolError
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_QUERY_COMMAND_NOT_FOUND => {
                MongoErrorCode::QueryCommandNotFound
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_QUERY_NOT_TAILABLE => {
                MongoErrorCode::QueryNotTailable
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_WRITE_CONCERN_ERROR => {
                MongoErrorCode::WriteConcernError
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_DUPLICATE_KEY => {
                MongoErrorCode::DuplicateKey
            }
            code => MongoErrorCode::Unknown(code),
        }
    }

    /// The error's message.
    pub fn get_message(&self) -> Cow<str> {
        let cstr = unsafe { CStr::from_ptr(&self.inner.message as *const i8) };
        String::from_utf8_lossy(cstr.to_bytes())
    }

    #[doc(hidden)]
    pub fn mut_inner(&mut self) -> &mut bindings::bson_error_t {
        &mut self.inner
    }
}

impl fmt::Debug for BsoncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BsoncError: {:?}/{:?} - {}",
            &self.domain(),
            &self.code(),
            &self.get_message()
        )
    }
}

impl fmt::Display for BsoncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl error::Error for BsoncError {
    fn description(&self) -> &str {
        "Error reported by the underlying Mongo C driver"
    }
}

impl From<BsoncError> for MongoError {
    fn from(error: BsoncError) -> MongoError {
        MongoError::Bsonc(error)
    }
}

/// Invalid params error that can be reported by the underlying C driver.
pub struct InvalidParamsError;

impl fmt::Debug for InvalidParamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidParamsError: Invalid params supplied")
    }
}

impl fmt::Display for InvalidParamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid params supplied")
    }
}

impl error::Error for InvalidParamsError {
    fn description(&self) -> &str {
        "Invalid params reported by the underlying Mongo C driver, no more information is available"
    }
}

impl From<InvalidParamsError> for MongoError {
    fn from(error: InvalidParamsError) -> MongoError {
        MongoError::InvalidParams(error)
    }
}

/// Error returned by a bulk operation that includes a report in the reply document.
#[derive(Debug)]
pub struct BulkOperationError {
    /// Returned error
    pub error: MongoError,
    /// Error report
    pub reply: Document,
}

impl fmt::Display for BulkOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bulk operation error {}", self.error)
    }
}

impl error::Error for BulkOperationError {
    fn description(&self) -> &str {
        "Error returned by a bulk operation that includes a report in the reply document"
    }
}

#[cfg(test)]
mod tests {
    use super::{BsoncError, MongoErrorCode, MongoErrorDomain};

    #[test]
    fn test_bson_error_empty() {
        let mut error = BsoncError::empty();
        assert!(error.is_empty());
        error.mut_inner().code = 1;
        assert!(!error.is_empty());
        error.mut_inner().domain = 1;
        error.mut_inner().code = 0;
        assert!(!error.is_empty());
    }

    #[test]
    fn test_bson_error_domain() {
        let mut error = BsoncError::empty();
        assert_eq!(MongoErrorDomain::Blank, error.domain());
        error.mut_inner().domain = 1;
        assert_eq!(MongoErrorDomain::Client, error.domain());
    }

    #[test]
    fn test_bson_error_code() {
        let mut error = BsoncError::empty();
        assert_eq!(MongoErrorCode::Blank, error.code());
        error.mut_inner().code = 1;
        assert_eq!(MongoErrorCode::StreamInvalidType, error.code());
    }
}
