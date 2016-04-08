extern crate rustc_serialize;
extern crate websocket;

use websocket::header::{WebSocketProtocol};
use websocket::client::request::Url;
use websocket::{Message, Sender, Receiver};

use rustc_serialize::json;
use rustc_serialize::{Encodable, Encoder};

use std::any::Any;
use std::io::{Read, Write};

use options::{Details, Options};

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

impl MessageType {
    pub fn to_u32(&self) -> u32 {
        match self {
           &MessageType::HELLO => 1,
           &MessageType::WELCOME => 2,
           &MessageType::ABORT => 3,
           &MessageType::CHALLENGE => 4,
           &MessageType::AUTHENTICATE => 5,
           &MessageType::GOODBYE => 6,
           &MessageType::HEARTBEAT => 7,
           &MessageType::ERROR => 8,
           &MessageType::PUBLISH => 16,
           &MessageType::PUBLISHED => 17,
           &MessageType::SUBSCRIBE => 32,
           &MessageType::SUBSCRIBED => 33,
           _ => unimplemented!(),
        }
    }
}

impl Encodable for MessageType {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_u32(*self as u32)
    }

}

pub enum SocketType {
    WEBSOCKET,
}

pub enum SerializerModes {
    /// Default should be JSON
    JSON,
    MSGPACK,
}

pub struct Serializer {
    //encoder: Fn,
    //decoder: Fn,
    /// the protocol identifier
    id: String,
    /// true if the data being serialized is binary, false otherwise
    binary:  bool,
    /// an enum referring to the type of serilizer
    mode: SerializerModes,
}

impl Serializer {
    pub fn new(mode: SerializerModes) -> Self {
        //TODO:: Add JSON
        match mode {
            SerializerModes::JSON => 
                Serializer {id: "json".to_string(), binary: false, mode: mode },
                _ => unimplemented!(),
        }
    }
}

pub struct Transport<R: Read, W: Write> {
    // TODO: some socket type (Read/Write traits?)
    reader: R,
    writer: W,
    /// serializer engine used to encode/decode messages
    serializer: Serializer,
}

struct EventMessage<A: Encodable, K: Encodable> {
    message_type: MessageType,
    id: u64,
    options: Options,
    topic: String,
    args: Vec<WampEncodable<A>>,
    kwargs: K,
}

impl<A: Encodable, K: Encodable> Encodable for EventMessage <A, K> { 
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
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

struct EventJoin {
    message_type: MessageType,
    realm: String,
    details: Details,
}

impl Encodable for EventJoin {
fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        // [self.message_type, self.id, self.options, self.topic, self.args, self.kwargs];
        s.emit_seq(3, |s| {
            try!(s.emit_seq_elt(0, |s| self.message_type.encode(s)));
            try!(s.emit_seq_elt(1, |s| self.realm.encode(s)));
            try!(s.emit_seq_elt(2, |s| self.details.encode(s)));
            Ok(())
        })
    }
}

macro_rules! wamp_encodable {
    ($($t:ident),+) => {
        #[derive(Debug)]
        enum WampEncodable<T> {
             $($t($t),)* 
             Generic(T),
        }

        impl<T: Encodable> Encodable for WampEncodable<T> {
            fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
                match self {
                    $(&WampEncodable::$t(ref value) => value.encode(s),)+
                    &WampEncodable::Generic(ref value) => value.encode(s),
                }
            }
        }

        impl<T: Encodable> WampEncodable<T> { }
        // Other clients can have this simplifier type alias
        // type WampEncodable = WampEncodable<()>;
    }
}

wamp_encodable!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, String, f32, f64, bool, char);

pub struct Client {
    url: String,
    realm: String,
}

impl Client {
    fn new(url: &str, realm: &str) -> Self {
        Client {url: String::from(url), realm: String::from(realm)}
    }

    fn connect(&self) {
        println!("starting...");

        let url = Url::parse(&*self.url).expect("malformed URL given");
        let mut request = websocket::Client::connect(url).expect("could not connect to the router");

        // Set the protocol for this connection
        let protocol_name = "wamp.2.json".to_string(); // wamp.2. +  Serializer.id
        let protocol = WebSocketProtocol(vec![protocol_name]);
        request.headers.set(protocol);

        // TODO: validate protocol header (to ensure correct serializer) from router
        let response = request.send().expect("failed to receive response from the router");
        // a NotFound error would show up here
        response.validate().expect("failed or could not validate repsonse from router");
        
        // get client and split it into sender and receiver components
        println!("{:?}", response.headers);
        let (mut sender, mut receiver) = response.begin().split();
        
        // Join WAMP Session
        let join_msg = json::encode(&EventJoin {
                                      message_type: MessageType::HELLO, 
                                      realm: self.realm.clone(), 
                                      details: Details::new() ,
                                  }).unwrap();

        println!("{:?}", join_msg);
        sender.send_message(&Message::text(join_msg));

        // send a PUBLISH message
        // TODO: specify binary payload option
        let mut args : Vec<WampEncodable<()>> = Vec::new();
        args.push(WampEncodable::u32(5 as u32));
        args.push(WampEncodable::String("hello!".to_string()));

        let event_msg = EventMessage {
            message_type: MessageType::PUBLISH,
            id: 42,
            options: Options {id: 1}, 
            topic: "com.myapp.topic1".to_string(),
            args: args,
            kwargs: Options {id: 2}, 
        };
        let message = json::encode(&event_msg).expect("could not encode event message");
        println!("{:?}", message);
        sender.send_message(&Message::text(message));
    }
}

//// TODO: make custom Result Error enum that can wrap IO op errors
//pub fn send<E: Encodable>(transport: Transport, msg: E) -> Result<(), String> {
//   unimplemented!(); 
//}
//

/// Generates a new ID for an event
fn new_id() -> u64 {
    unimplemented!();
}

#[test]
fn naive_connect() {
    Client::new("ws://localhost:8080/ws", "realm1").connect();
}

#[test]
fn test_message_type_enum() {
    assert!(MessageType::HELLO.to_u32() == 1);
}

#[test]
fn decode_event_message() {
    // TODO: Test the correct decoding of this
    let msg : EventMessage<(), i32> = EventMessage {message_type: MessageType::HELLO, 
                            id: 42, options: Options {id: 1}, 
                            topic: "hello".to_string(), 
                            args: vec![WampEncodable::u32(5 as u32), WampEncodable::String("hello_world".to_string())], 
                            kwargs: -42};
    let msg_string = json::encode(&msg);
    println!("{:?}", msg_string);
}


