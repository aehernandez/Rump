//! A [WAMP](http://wamp-proto.org/) Client built on pure Rust. 
//!
//! Currently only bare Publish and Subcribe events are implemented.
//! Many standard features are also missing.
//!
//! # Examples
//!
//! Publish and subscribe to two sample topics on a realm.
//!
//! ```
//! use rump::client::Client;
//! use rump::message::WampType;
//! use std::collections::HashMap;
//!
//! // connect to a local router at the realm "realm1"
//! let mut session = Client::new("ws://localhost:8080/ws", "realm1").connect().unwrap(); 
//!
//! let mut kwarg_map = HashMap::new();
//! kwarg_map.insert("some_key", "some_value");
//! kwarg_map.insert("another_key", "another_value");
//!
//! let args = vec![WampType::i32(42), WampType::String("hello from rust!".to_string())];
//! let kwargs = WampType::Map(kwarg_map);
//!
//! // publish the positional args (42, "hello from rust!") and the keyword argument
//! // {"some_key": "some_value", "another_key", "another_value"} on the sample topic URI
//! session.publish("com.myapp.topic1", args, kwargs); 
//! 
//! // If we wanted to write a complementary client to receive the published argument,
//! // we can use the following code ...
//!
//! // Our sample struct to receive the kwarg map
//! // Note: it must have the RustcDecodable trait from rustc-serialize
//! #[derive(RustcDecodable)]
//! struct TestStruct {
//!     some_key: String,
//!     another_key: String,
//! }
//!
//! // define a callback to be called when the event is published
//! let callback = |payload| {
//!     // get the arguments from the payload
//!     let (value, greeting) : (i64, String) = payload.decode_args().unwrap();
//!     let sample_map : TestStruct = payload.decode_kwargs().unwrap();
//!
//!     // we can do some other work with this function...
//! };
//!                                         
//! session.subscribe("com.myapp.topic2", callback);
//! ```  
//!

extern crate rustc_serialize;
extern crate websocket;
extern crate crossbeam;
extern crate rand;

pub mod client;
mod options;
mod transport;
mod message;

use std::result;
use websocket::result::WebSocketError;
use std::sync::mpsc::SendError;
use rustc_serialize::json;

// Some re-exports
pub use message::WampType;
pub use message::Payload;

#[derive(Debug)]
pub enum WampError {
    InvalidURL,
    WebSocketError(WebSocketError),
    InternalThreadError,
    ProtocolError,
    DecodeError (json::DecoderError),
}

//impl From<ParseError> for WampError {
//    fn from(err: ParseError) -> WampError{
//        WampError::InvalidURL(err)
//    }
//}

impl <T> From<SendError<T>> for WampError {
    fn from(err: SendError<T>) -> WampError {
        WampError::InternalThreadError
    }
}

impl From<WebSocketError> for WampError {
    fn from(err: WebSocketError) -> WampError{
        WampError::WebSocketError(err)
    }
}

pub type WampResult<T> = result::Result<T, WampError>;
