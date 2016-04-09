extern crate rustc_serialize;
extern crate websocket;
extern crate crossbeam;

use websocket::header::{WebSocketProtocol};
use websocket::client::request::Url;
use websocket::{message, Message, Sender, Receiver};

use std::{thread};
use std::sync::mpsc;

use rustc_serialize::{Encodable, Encoder};
use rustc_serialize::json;

use std::io::{self, Read, Write};
use std::result;
use options::{Details, Options};

use super::WampResult;
use super::WampError;

//TODO: Give the following enums their write numeric value
#[derive(Copy, Clone)]
pub enum MessageType {
    HELLO = 1,
    WELCOME = 2,
    ABORT = 3,
    CHALLENGE,
    AUTHENTICATE,
    GOODBYE,
    HEARTBEAT,
    ERROR,
    PUBLISH = 16,
    PUBLISHED,
    SUBSCRIBE,
    SUBSCRIBED,
    UNSUBSCRIBE,
    UNSUBSCRIBED,
    EVENT,
    CALL,
    CANCEL,
    RESULT,
    REGISTER,
    REGISTERED,
    UNREGISTER,
    UNREGISTERED,
    INVOCATION,
    INTERRUPT,
    YIELD
} 

impl Encodable for MessageType {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        s.emit_u32(*self as u32)
    }

}

pub enum SocketType {
    WEBSOCKET,
}

pub enum SerializerType {
    /// Default should be JSON
    JSON,
    //MSGPACK,
}

pub fn new_event_id() -> u64 {
    // TODO: Randomely generate this...
    42
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

    pub fn encode<T: Encodable>(&self, message: T) -> Message {
        match self.mode {
            SerializerType::JSON => Message::text(json::encode(&message).unwrap())
        }
    }
}

pub trait WampConnector {
    fn connect<F>(url: String, serializer: Serializer, on_message: F) -> WampResult<Self> where Self : Sized, F: Fn(Message) + Send;
}

pub trait WampSender : WampConnector {
    fn send<'a, T: Encodable>(&'a mut self, message: &T) -> WampResult<()>;
}

pub struct WebSocket<'a> {
    sender: mpsc::Sender<Message<'a>>,
    serializer: Serializer
}

impl <'a> WampConnector for WebSocket<'a>  {
    fn connect<F>(url: String, serializer: Serializer, on_message: F) -> WampResult<Self> 
        where F:'a + Fn(Message) + Send {
        let url = try!(Url::parse(&*url).map_err(|e| WampError::InvalidURL));
        let mut request = try!(websocket::Client::connect(url));
        let protocol_name = "wamp.2.".to_string() + &*serializer.id;
        let protocol = WebSocketProtocol(vec![protocol_name]);
        request.headers.set(protocol);

        let response = try!(request.send());
        try!(response.validate());

        let (mut sender, mut receiver) = response.begin().split();

        let (tx, rx) = mpsc::channel();
        let receive_tx = tx.clone();

        crossbeam::scope(move |scope| {
            scope.spawn(move || {
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

            scope.spawn(move || {
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
                        message::Type::Ping => match receive_tx.send(Message::pong(message.payload)) {
                            // Send a pong in response
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                return;
                            }
                        },
                        // Say what we received
                        _ => println!("Receive Loop: {:?}", message),
                    }

                    // let the client handle the message
                    //on_message(message);
                }
            });
        });

        Ok(WebSocket {sender: tx, 
            serializer: serializer,
        })
    }
}

impl <'a> WampSender for WebSocket<'a>  {
    fn send<T: Encodable>(&mut self, message: &T) -> WampResult<()> {
        self.sender.send(self.serializer.encode(message));
        Ok(())
    }
}

pub struct EventMessage<A: Encodable, K: Encodable> {
    pub message_type: MessageType,
    pub id: u64,
    pub options: Options,
    pub topic: String,
    pub args: Vec<WampEncodable<A>>,
    pub kwargs: K,
}

impl<A: Encodable, K: Encodable> Encodable for EventMessage <A, K> { 
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        // [self.message_type, self.id, self.options, self.topic, self.args, self.kwargs];
        s.emit_seq(6, |s| {
            try!(s.emit_seq_elt(0, |s| self.message_type.encode(s)));
            try!(s.emit_seq_elt(1, |s| self.id.encode(s)));
            try!(s.emit_seq_elt(2, |s| self.options.encode(s)));
            try!(s.emit_seq_elt(3, |s| self.topic.encode(s)));
            try!(s.emit_seq_elt(4, |s| self.args.encode(s)));
            try!(s.emit_seq_elt(5, |s| self.kwargs.encode(s)));
            Ok(())
        })
    }
}

pub struct EventJoin {
    pub message_type: MessageType,
    pub realm: String,
    pub details: Details,
}


impl Encodable for EventJoin {
fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        // [self.message_type, self.id, self.options, self.topic, self.args, self.kwargs];
        s.emit_seq(3, |s| {
            try!(s.emit_seq_elt(0, |s| self.message_type.encode(s)));
            try!(s.emit_seq_elt(1, |s| self.realm.encode(s)));
            try!(s.emit_seq_elt(2, |s| self.details.encode(s)));
            Ok(())
        })
    }
}

//TODO: better naming scheme for WampEncodable variants
macro_rules! wamp_encodable {
    ($($t:ident),+) => {
        #[derive(Debug)]
        pub enum WampEncodable<T> {
             $($t($t),)* 
             Generic(T),
             None,
        }

        impl<T: Encodable> Encodable for WampEncodable<T> {
            fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
                match self {
                    $(&WampEncodable::$t(ref value) => value.encode(s),)+
                    &WampEncodable::Generic(ref value) => value.encode(s),
                    &WampEncodable::None => s.emit_map(0, |s| Ok(())),
                }
            }
        }

        impl<T: Encodable> WampEncodable<T> { }
        // Other clients can have this simplifier type alias
        // type WampEncodable = WampEncodable<()>;
    }
}

wamp_encodable!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, String, f32, f64, bool, char);


