extern crate rustc_serialize;
extern crate rand;

use rand::Rng;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

use options::{Options, Details};
use transport::Serializer;

use std::result;
use std::collections::HashMap;

use WampError;
use WampResult;


/// All WAMP events and message types and their numeric counterparts
#[derive(Copy, Clone, Debug, PartialEq)]
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
    NONE = 0 
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

//#[derive(Debug)]
pub enum WampEvent {
    Subscribed {
        message_type: MessageType,
        event_id: u64,
        topic_id: u64,
    },
    Event {
        message_type: MessageType,
        topic_id: u64,
        event_id: u64,
        options: Options,
        has_kwargs: bool,
    },
}

impl Decodable for WampEvent {
    fn decode<D: Decoder>(d: &mut D) -> Result<WampEvent, D::Error> {
        d.read_seq(|d, len| {
            if len > 0 {
                let message_type = try!(d.read_seq_elt(0, |d| d.read_u32()));
                let message_type = MessageType::from(message_type);
                match message_type {
                    MessageType::SUBSCRIBED => {
                        if len != 3 {
                            Err(d.error("unexpected len != 3 for SUBSCRIBED message"))
                        } else {
                            let event_id = try!(d.read_seq_elt(1, |d| d.read_u64()));
                            let topic_id = try!(d.read_seq_elt(2, |d| d.read_u64()));
                            Ok(WampEvent::Subscribed {
                                message_type: message_type,
                                event_id: event_id,
                                topic_id: topic_id,
                            })
                        }
                    },

                    MessageType::EVENT => {
                        if len != 5 && len != 6 {
                            Err(d.error("unexpected len != {5, 6} for EVENT message"))
                        } else {
                            let topic_id = try!(d.read_seq_elt(1, |d| d.read_u64()));
                            let event_id = try!(d.read_seq_elt(2, |d| d.read_u64()));
                            //TODO: Actually fulfill these options!
                            let options = Options::Empty;
                            let has_kwargs = len == 6;

                            // Read positional arguments
                            //let args = try!(d.read_seq_elt(4, |d| d.read_seq(|d, len| {
                            //   let mut v : Vec<WampType> = Vec::with_capacity(len);
                            //   for i in 0..len {
                            //    v.push(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
                            //   }
                            //   Ok(v)
                            //})));
                            
                           Ok(WampEvent::Event {
                               message_type: message_type, 
                               topic_id: topic_id,
                               event_id: event_id,
                               options: options,
                               has_kwargs: has_kwargs, 
                           })
                        }
                    }
                    _ => Err(d.error(&*format!("protocol violation: no message type {} exists", 
                                               message_type as u32)))
                }
            } else {
                // bad things happen
                Err(d.error("empty message received"))
            }
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
/// A struct representing the payload that's received from a WAMP event.
pub struct Payload {
    args: String,
    kwargs: Option<String>,
    serializer: Serializer
}

impl Payload {
    fn capture_braces(raw: &str, braces: (char, char)) -> Option<(usize, usize)> {
        let left_brace = braces.0;
        let right_brace = braces.1;
        assert!(left_brace != right_brace);

        let mut first_index : usize  = 0;
        let mut final_index : usize  = 0;
        let mut found_initial_brace = false;
        let mut count = -1;

        for (i, c) in raw.char_indices() {

            if c == left_brace { 
                count = count + 1;
                if !found_initial_brace {
                    first_index = i;
                    found_initial_brace = true;
                }
            } else if c == right_brace {
                if found_initial_brace {
                    if count == 0 {
                        final_index = i; 
                        break;
                    } else {
                        count = count - 1;
                    }
                }
            }
        }

        if count == 0 && found_initial_brace && final_index > first_index {
            Some((first_index, final_index))
        } else {
            None
        }
    }

    /// Parse args and kwargs from a WAMP Message that is known to have a payload
    pub fn from_str(raw: &str) -> WampResult<Payload> {
        if let Some((_, end)) = Self::capture_braces(raw, ('{', '}')) {
            let (_, next) = raw.split_at(end + 1);
            if let Some((args_l, args_r)) = Self::capture_braces(next, ('[', ']')) {
                let args = &next[args_l .. args_r + 1];
                let (_, kw_next) = next.split_at(args_r);
                let kwargs = Self::capture_braces(kw_next, ('{', '}'))
                    .map(|(kwargs_l, kwargs_r)| kw_next[kwargs_l .. kwargs_r + 1].to_string());

                return Ok(Payload {
                    args: args.to_string(),
                    kwargs: kwargs,
                    serializer: Serializer::json()
                });
            }
        } 

        Err(WampError::ProtocolError)
    }


    #[allow(dead_code)]
    /// Extract positional arguments from the payload.
    pub fn decode_args<T: Decodable>(&self) -> WampResult<T> {
        self.serializer.decode(&*self.args)
    }

    #[allow(dead_code)]
    /// Extract keyword arguments from the payload.
    pub fn decode_kwargs<T: Decodable>(&self) -> WampResult<T> {
        self.serializer.decode(&**self.kwargs.as_ref().unwrap_or(&String::from(""))) 
    }

    #[allow(dead_code)]
    pub fn has_args(&self) -> bool {
        self.args.len() > 0
    }

    #[allow(dead_code)]
    pub fn has_kwargs(&self) -> bool {
        self.kwargs.is_some()
    }
}


impl Encodable for MessageType {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        s.emit_u32(*self as u32)
    }
}

/// Generates a new event_id to track messages sent to and from the WAMP Router
pub fn new_event_id() -> u64 {
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
    pub args: Vec<A>,
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
    message_type: MessageType,
    id: u64,
    options: Options,
    topic: String,
}

impl EventSubscribe {
    pub fn new(topic: String) -> Self {
        EventSubscribe {
            message_type: MessageType::SUBSCRIBE,
            id: new_event_id(),
            topic: topic,
            options: Options::Empty
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
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
pub struct EventJoin {
    pub message_type: MessageType,
    pub realm: String,
    pub details: Details,
}

impl EventJoin {
    pub fn new (realm: String) {
        unimplemented!();
    }
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

macro_rules! wamp_type {
    ($($t:ident),+) => {
        /// All types that can be sent to/from a WAMP Router
        /// Used to publish hetereogenous arguments
        #[allow(dead_code, non_camel_case_types)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum WampType {
            $($t($t),)* 
                Vec(Vec<WampType>),
                Map(HashMap<String, WampType>),
                /// Used to send an empty struct or keymap value "{}"
                None, 
        }

        impl Encodable for WampType {
            fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
                match self {
                    $(&WampType::$t(ref value) => value.encode(s),)+
                        &WampType::Vec(ref value) => value.encode(s),
                        &WampType::Map(ref value) => value.encode(s),
                        &WampType::None => s.emit_map(0, |_| Ok(())),
                }
            }
        }

        // impl Decodable for WampType {
        //     fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        //         unimplemented!();
        //     }
        // }
        
        // Other clients can have this simplifier type alias
        // type WampEncodable = WampEncodable<()>;
    }
}

// TODO: add types per request
wamp_type!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, String, f32, f64, bool, char);

#[test]
fn message_enum_value() {
    assert!(MessageType::HELLO as u32 == 1);
    assert!(MessageType::SUBSCRIBE as u32 == 32);
}

#[test]
fn message_payload_braces() {
    let simple_brace = "[hello, world]"; 
    assert!(Payload::capture_braces(simple_brace, ('[', ']')) == Some((0, simple_brace.len()-1)));
    assert!(Payload::capture_braces("[[hello], [test, [thing]]], [other, [stuff]]", ('[',']')) 
                                    == Some((0, 25))); 
}

#[test]
fn message_extract_payload() {
    let sample_message_nokwargs = "[36,1232131,64713717171,{},[42, \"yup\"]]";
    let sample_payload = Payload::from_str(sample_message_nokwargs);
    let (number, yup) : (u32, String) = sample_payload.unwrap().decode_args().unwrap();
    assert!(number == 42);
    assert!(yup == "yup".to_string());

    let message_kwargs_only = "[42, 12415261, 16171, {},[],{\"field\": 42, \"binary\": false, \"word\": \"hello world\"}]";
    #[derive(PartialEq, RustcDecodable)]
    struct TestStruct {
        field: u32,
        binary: bool,
        word: String,
    }
    let payload2 = Payload::from_str(message_kwargs_only);
    let test_struct: TestStruct= payload2.unwrap().decode_kwargs().unwrap();
    assert!(test_struct == TestStruct{field: 42, binary: false, word: "hello world".to_string()});
}
