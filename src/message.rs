extern crate rustc_serialize;

use rustc_serialize::{Encodable, Encoder};
use options::{Options, Details};
use std::result;

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
pub fn new_event_id() -> u64 {
    // TODO: Randomely generate this...
    42
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


