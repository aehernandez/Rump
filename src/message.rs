extern crate rustc_serialize;
extern crate rand;

use rand::Rng;
use rustc_serialize::{Encodable, Encoder};
use options::{Options, Details};
use std::result;
use std::any::Any;

/// All WAMP events and message types and their numeric counterparts
#[derive(Copy, Clone, Debug)]
pub enum MessageType {
    HELLO = 1,
    WELCOME,
    ABORT,
    CHALLENGE,
    AUTHENTICATE,
    GOODBYE,
    HEARTBEAT,
    ERROR,
    PUBLISH = 16,
    PUBLISHED,
    SUBSCRIBE = 32,
    SUBSCRIBED,
    UNSUBSCRIBE,
    UNSUBSCRIBED,
    EVENT,
    CALL = 48,
    CANCEL,
    RESULT,
    REGISTER = 64,
    REGISTERED,
    UNREGISTER,
    UNREGISTERED,
    INVOCATION,
    INTERRUPT,
    YIELD,
    /// The message type value was not found, usually this means a protocol violation occured.
    NONE = -1
} 

impl From<u32> for MessageType {
    fn from(value: u32) -> Self {
        match value {
            1 => MessageType::HELLO,
            2 => MessageType::WELCOME,
            3 => MessageType::ABORT,
            4 => MessageType:: CHALLENGE,
            5 => MessageType::AUTHENTICATE,
            6 => MessageType::GOODBYE,
            7 => MessageType::HEARTBEAT,
            8 => MessageType::ERROR,
            16 => MessageType::PUBLISH,
            17 => MessageType::PUBLISHED,
            32 => MessageType::SUBSCRIBE,
            33 => MessageType::SUBSCRIBED,
            34 => MessageType::UNSUBSCRIBE,
            35 => MessageType::UNSUBSCRIBED,
            36 => MessageType::EVENT,
            48 => MessageType::CALL,
            49 => MessageType::CANCEL,
            50 => MessageType::RESULT,
            64 => MessageType::REGISTER,
            65 => MessageType::REGISTERED,
            66 => MessageType::UNREGISTER,
            67 => MessageType::UNREGISTERED,
            68 => MessageType::INVOCATION,
            69 => MessageType::INTERRUPT,
            70 => MessageType::YIELD,
            _ => MessageType::NONE,
        }
    }
}

pub enum WampEvent {
    Subscribed {
        message_type: MessageType,
        event_id: u64,
        topic_id: u64,
    },
    Subscription {
        message_type: MessageType,
        topic_id: u64,
        Options: Options,
        args: Vec<Box<Any>>,
        kwargs: Option<Box<Any>>,
    }
}

pub fn decode_event(raw: &str) -> WampEvent {
    unimplemented!();
}
pub fn get_event_type(raw: &str) -> MessageType {
    unimplemented!();
}


impl Encodable for MessageType {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        s.emit_u32(*self as u32)
    }
}

/// Generates a new event_id to track messages sent to and from the WAMP Router
pub fn new_event_id() -> u64 {
    // TODO: Randomely generate this...
    rand::thread_rng().next_u32() as u64
}


//TODO: Currently every message type has its own struct,
// perhaps there is a better way to abstract this with an enum
// however, message types should never be directly exposed to users of the library
//

#[derive(Debug, Clone)]
pub struct EventPublish<A: Encodable, K: Encodable> {
    pub message_type: MessageType,
    pub id: u64,
    pub options: Options,
    pub topic: String,
    pub args: Vec<WampEncodable<A>>,
    pub kwargs: K,
}

impl<A: Encodable, K: Encodable> Encodable for EventPublish<A, K> { 
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

#[derive(Debug, Clone)]
pub struct EventSubscribe {
    pub message_type: MessageType,
    pub id: u64,
    pub options: Options,
    pub topic: String,
}


impl Encodable for EventSubscribe { 
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        // [self.message_type, self.id, self.options, self.topic, self.args, self.kwargs];
        s.emit_seq(4, |s| {
            try!(s.emit_seq_elt(0, |s| self.message_type.encode(s)));
            try!(s.emit_seq_elt(1, |s| self.id.encode(s)));
            try!(s.emit_seq_elt(2, |s| self.options.encode(s)));
            try!(s.emit_seq_elt(3, |s| self.topic.encode(s)));
            Ok(())
        })
    }
}

#[derive(Debug, Clone)]
pub struct EventSubscribed {
    pub message_type: MessageType,
    pub id: u64,
    pub topic: u64,
}

#[derive(Debug, Clone)]
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

macro_rules! wamp_encodable {
    ($($t:ident),+) => {
        /// All types that can be sent to/from a WAMP Router
        /// Used to publish non-hetereogenous positional arguments
        #[derive(Debug, Clone)]
        pub enum WampEncodable<T> {
            $($t($t),)* 
                /// Used to send a custom user-defined `Encodable` type.
                Generic(T), 
                /// Used to send an empty struct or keymap value "{}"
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

// TODO: add types per request
wamp_encodable!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, String, f32, f64, bool, char);

#[test]
fn message_enum_value() {
    assert!(MessageType::HELLO as u32 == 1);
    assert!(MessageType::SUBSCRIBE as u32 == 32);
    assert!((1 as u32) as MessageType == MessageType::HELLO);
}
