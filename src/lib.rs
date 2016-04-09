extern crate rustc_serialize;
extern crate websocket;
extern crate crossbeam;
extern crate rand;

pub mod client;
mod options;
mod transport;

use std::result;
use websocket::result::WebSocketError;
use std::sync::mpsc::SendError;

#[derive(Debug)]
pub enum WampError {
    InvalidURL,
    WebSocketError(WebSocketError),
    InternalThreadError,
}

//impl From<ParseError> for WampError {
//    fn from(err: ParseError) -> WampError{
//        WampError::InvalidURL(err)
//    }
//}

impl From<WebSocketError> for WampError {
    fn from(err: WebSocketError) -> WampError{
        WampError::WebSocketError(err)
    }
}

impl <T> From<SendError<T>> for WampError {
    fn from(err: SendError<T>) -> WampError {
        WampError::InternalThreadError
    }
}

pub type WampResult<T> = result::Result<T, WampError>;
