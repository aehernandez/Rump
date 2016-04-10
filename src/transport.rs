extern crate rustc_serialize;
extern crate websocket;

use std::{thread};
use std::sync::mpsc;

use websocket::header::{WebSocketProtocol};
use websocket::client::request::Url;
use websocket::{message, Message, Sender, Receiver};

use rustc_serialize::{Encodable, Encoder};
use rustc_serialize::json;

use super::WampResult;
use super::WampError;

/// A type enumerating all the possible underlying socket implementations.
pub enum SocketType {
    WEBSOCKET,
}

/// A type enumerating all possible serialization engines
pub enum SerializerType {
    /// JSON can be used for human-readable structured data
    JSON,
//    /// [MsgPack](http://msgpack.org/index.html) can be used for binary data and structured data
    //MSGPACK,
}


pub struct Serializer {
    id: String,
    /// true if the data being serialized is binary, false otherwise
    binary:  bool,
    /// an enum referring to the type of serilizer
    mode: SerializerType,
}

impl Serializer {
    pub fn new(mode: SerializerType) -> Self {
        match mode {
            SerializerType::JSON => 
                Serializer {id: "json".to_string(), binary: false, mode: mode}
                            
        }
    }

    /// Serialize an encodable message into one that can be sent over a socket
    pub fn encode<'a, T: Encodable>(&self, message: &T) -> Message<'a> {
        match self.mode {
            SerializerType::JSON => Message::text(json::encode(message).unwrap())
        }
    }
}

pub trait WampConnector {
    fn connect<F>(url: String, serializer: Serializer, on_message: F) -> WampResult<Self> where Self : Sized, F: Fn(Message) + Send;
}

pub trait WampSender : WampConnector {
    fn send<T: Encodable>(&self, message: &T) -> WampResult<()>;
}

pub struct WebSocket {
    sender: mpsc::Sender<Message<'static>>,
    serializer: Serializer
}

impl WampConnector for WebSocket {
    //TODO: 'static lifetime for this function, is this valid?
    fn connect<F>(url: String, serializer: Serializer, on_message: F) -> WampResult<Self> 
        where F:'static + Fn(Message) + Send {
        let url = try!(Url::parse(&*url).map_err(|e| WampError::InvalidURL));
        let mut request = try!(websocket::Client::connect(url));
        let protocol_name = "wamp.2.".to_string() + &*serializer.id;
        let protocol = WebSocketProtocol(vec![protocol_name]);
        request.headers.set(protocol);

        let response = try!(request.send());
        try!(response.validate());

        let (mut sender, mut receiver) = response.begin().split();

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                // Send loop
                let message: Message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };

                // Send the message
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        });

        let receive_tx = tx.clone();
        thread::spawn(move || {
            // TODO: messages received are on a single thread,
            // rust-weboscket may eventually may to a multi-threaded model, which
            // may break this current implementation
            // Receive loop
            for message in receiver.incoming_messages() {
                let message: Message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        return;
                    }
                };

                // Handle the message on the socket side
                match message.opcode {
                    message::Type::Close => {
                        // TODO: Handle this on the session
                        // Got a close message, so send a close message and return
                        let _ = receive_tx.send(Message::close());
                        return;
                    }
                    // Say what we received
                    _ => println!("Receive Loop: {:?}", message),
                }

                // let the client handle the message
                on_message(message);
            }
        });

        Ok(WebSocket {
            sender: tx, 
            serializer: serializer
        })
    }
}

impl WampSender for WebSocket {
    fn send<T: Encodable>(&self, message: &T) -> WampResult<()> {
        let event = self.serializer.encode(message);
        try!(self.sender.send(event));
        Ok(())
    }
}
