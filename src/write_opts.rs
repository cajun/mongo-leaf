/// NOT WORKING
/// See: http://mongoc.org/libmongoc/current/mongoc_client_session_append.html to create opts
//use crate::{
//bindings,
//error::Result,
//session::Session,
//write_concern::{WriteConcern, WriteConcernLevel, WriteConcernc},
//};
//use bson;
//use std::ptr;

//struct WriteOpts {
//write_level: *const bindings::mongoc_write_concern_t,
//session_id: *mut bindings::mongoc_client_session_t,
//validate: Option<*mut bindings::bson_validate_flags_t>,
//ordered: bool,
//bypass_document_validation: bool,
//}

//impl WriteOpts {
//pub fn new() -> WriteOpts {
//let wc = WriteConcernc::default();
//WriteOpts {
//write_level: wc.as_ptr(),
//session_id: ptr::null_mut(),
//validate: None,
//ordered: false,
//bypass_document_validation: false,
//}
//}

//pub fn set_write_level(&mut self, level: WriteConcernLevel) -> &Self {
//let wc = WriteConcernc::new(level, None);
//self.write_level = wc.as_ptr();
//self
//}

//pub fn set_session(&mut self, session: impl Session) -> &Self {
//self.session_id = session.as_mut_ptr();
//self
//}

//pub fn set_order(&mut self, order: bool) -> &Self {
//self.ordered = order;
//self
//}

//pub fn set_validate(&mut self, validate: *mut bindings::bson_validate_flags_t) -> &Self {
//self.validate = Some(validate);
//self
//}

//pub fn set_bypass_document_validation(&mut self, bypass: bool) -> &Self {
//self.bypass_document_validation = bypass;
//self
//}

//pub fn as_document(&self) -> Result<&bson::Document> {
//let bson_doc = bson::to_bson(self)?;
//Ok(bson_doc.as_document().expect("should be valid document"))
//}
//}
