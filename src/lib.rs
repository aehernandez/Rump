extern crate rustc_serialize;
extern crate websocket;
extern crate rand;

pub mod client;
mod options;
mod transport;

use std::result;
use websocket::result::WebSocketError;

#[derive(Debug)]
pub enum WampError {
    InvalidURL,
    WebSocketError(WebSocketError),
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

pub type WampResult<T> = result::Result<T, WampError>;
