use crate::{bindings, read_concern::ReadConcernLevel};
use std::borrow::Cow;
use std::error;
use std::ffi::CStr;
use std::fmt;

use bson::{DecoderError, Document, EncoderError, ValueAccessError};
use failure::{Backtrace, Context, Fail};
use std::ffi::NulError;

pub type Result<LeafError> = std::result::Result<LeafError, failure::Error>;

#[derive(Debug)]
struct LeafError {
    inner: Context<MongoError>,
}

/// Wrapper for all errors that can occur in the driver.
#[derive(Fail)]
pub enum MongoError {
    /// Error in the underlying C driver.
    Bsonc(Box<BsoncError>),
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
    InvalidReadConcern(ReadConcernLevel),
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
            MongoError::InvalidReadConcern(ref err) => write!(f, "Invalid Read concern of {}", err),
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
            MongoError::InvalidReadConcern(ref err) => {
                write!(f, "MongoError (Invalid Read concern of {:?})", err)
            }
        }
    }
}

impl Fail for LeafError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for LeafError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<MongoError> for LeafError {
    fn from(kind: MongoError) -> LeafError {
        LeafError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<MongoError>> for LeafError {
    fn from(inner: Context<MongoError>) -> LeafError {
        LeafError { inner }
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
    ServerSelection,
    WriteConcern,
    Server,
    Transaction,
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
    ServerSelectionBadWireVersion,
    ServerSelectionFailure,
    ServerSelectionInvalidId,
    GridfsChunkMissing,
    GridfsProtocolError,
    ProtocolError,
    MaxTimeMsExpired,
    ChangeStreamNoResumeToken,
    ClientSessionFailure,
    TransactionInvalidState,
    GridfsCorrupt,
    GridfsBucketFileNotFound,
    GridfsBucketFtream,
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
            bindings::mongoc_error_domain_t_MONGOC_ERROR_SERVER_SELECTION => {
                MongoErrorDomain::ServerSelection
            }
            bindings::mongoc_error_domain_t_MONGOC_ERROR_WRITE_CONCERN => {
                MongoErrorDomain::WriteConcern
            }
            bindings::mongoc_error_domain_t_MONGOC_ERROR_SERVER => MongoErrorDomain::Server,
            bindings::mongoc_error_domain_t_MONGOC_ERROR_TRANSACTION => {
                MongoErrorDomain::Transaction
            }
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

            bindings::mongoc_error_code_t_MONGOC_ERROR_SERVER_SELECTION_BAD_WIRE_VERSION => {
                MongoErrorCode::ServerSelectionBadWireVersion
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_SERVER_SELECTION_FAILURE => {
                MongoErrorCode::ServerSelectionFailure
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_SERVER_SELECTION_INVALID_ID => {
                MongoErrorCode::ServerSelectionInvalidId
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_GRIDFS_CHUNK_MISSING => {
                MongoErrorCode::GridfsChunkMissing
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_GRIDFS_PROTOCOL_ERROR => {
                MongoErrorCode::GridfsProtocolError
            }
            //bindings::mongoc_error_code_t_MONGOC_ERROR_PROTOCOL_ERROR => {
            //MongoErrorCode::ProtocolError
            //}
            bindings::mongoc_error_code_t_MONGOC_ERROR_MAX_TIME_MS_EXPIRED => {
                MongoErrorCode::MaxTimeMsExpired
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CHANGE_STREAM_NO_RESUME_TOKEN => {
                MongoErrorCode::ChangeStreamNoResumeToken
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_CLIENT_SESSION_FAILURE => {
                MongoErrorCode::ClientSessionFailure
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_TRANSACTION_INVALID_STATE => {
                MongoErrorCode::TransactionInvalidState
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_GRIDFS_CORRUPT => {
                MongoErrorCode::GridfsCorrupt
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_GRIDFS_BUCKET_FILE_NOT_FOUND => {
                MongoErrorCode::GridfsBucketFileNotFound
            }
            bindings::mongoc_error_code_t_MONGOC_ERROR_GRIDFS_BUCKET_STREAM => {
                MongoErrorCode::GridfsBucketFtream
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
    pub fn as_mut_ptr(&mut self) -> &mut bindings::bson_error_t {
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
        MongoError::Bsonc(Box::new(error))
    }
}

/// Invalid params error that can be reported by the underlying C driver.
#[derive(Fail, Debug)]
#[fail(display = "Invalid params supplied")]
pub struct InvalidParamsError;

impl From<InvalidParamsError> for MongoError {
    fn from(error: InvalidParamsError) -> MongoError {
        MongoError::InvalidParams(error)
    }
}

/// Error returned by a bulk operation that includes a report in the reply document.
#[derive(Fail, Debug)]
#[fail(display = "Bulk operation error {}", error)]
pub struct BulkOperationError {
    /// Returned error
    pub error: MongoError,
    /// Error report
    pub reply: Document,
}

#[cfg(test)]
mod tests {
    use super::{BsoncError, MongoErrorCode, MongoErrorDomain};

    #[test]
    fn test_bson_error_empty() {
        let mut error = BsoncError::empty();
        assert!(error.is_empty());
        error.as_mut_ptr().code = 1;
        assert!(!error.is_empty());
        error.as_mut_ptr().domain = 1;
        error.as_mut_ptr().code = 0;
        assert!(!error.is_empty());
    }

    #[test]
    fn test_bson_error_domain() {
        let mut error = BsoncError::empty();
        assert_eq!(MongoErrorDomain::Blank, error.domain());
        error.as_mut_ptr().domain = 1;
        assert_eq!(MongoErrorDomain::Client, error.domain());
    }

    #[test]
    fn test_bson_error_code() {
        let mut error = BsoncError::empty();
        assert_eq!(MongoErrorCode::Blank, error.code());
        error.as_mut_ptr().code = 1;
        assert_eq!(MongoErrorCode::StreamInvalidType, error.code());
    }
}
