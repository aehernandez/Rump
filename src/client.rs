extern crate rustc_serialize;
extern crate websocket;

use transport::{WampSender, WampConnector, WebSocket, Serializer};
use message::{WampEvent, MessageType, EventPublish, EventJoin, Payload, EventSubscribe, new_event_id};
use options::{Options, Details};

use rustc_serialize::{Encodable, Decodable};

use websocket::{Message};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use super::{WampResult};

use std::str::from_utf8;

use std::borrow::Borrow;

/// A Client defines methods and options for building a Session with a WAMP Router
///
/// # Examples
///
/// To obtain a Session from a client with default settings perform the following:
///
/// ```
/// # extern crate rump;
/// # use rump::client::Client;
/// # fn main() {
/// let session = Client::new("ws://router_address:port/ws", "realm_name").connect().unwrap();
/// # }
/// ```
///
pub struct Client {
    url: String,
    realm: String,
}

enum SessionState {
    NotConnected,
    Connected,
}

/// A Session represents a valid WAMP Session with a Router. 
/// You can obtain a `Session` from a `Client`
pub struct Session <S: WampSender> {
    /// Outgoing socket connection to WAMP Router
    sender: S,
    /// Map topic URIs to the event_id that generated them and corresponding callback
    pending_subscriptions: Arc<Mutex<HashMap<u64, (String, Box<Fn(&Payload) + Send>)>>>,
    /// Two maps: Firstly a mapping from topic IDs to topic URIs
    /// Secondly, topic URIs to their callbacks 
    subscriptions: Arc<Mutex<(HashMap<u64, String>, HashMap<String, Vec<Box<Fn(&Payload) + Send>>>)>>,
}

impl <S: WampSender> Session<S> {
    /// Connects to a WAMP Router in a realm without authentication
    pub fn join(&self, realm: String) -> WampResult<()> {
        let join_msg = EventJoin { 
            message_type: MessageType::HELLO,
            realm: realm.clone(),
            details: Details::new(),
        };

        self.sender.send(&join_msg)
    }

    /// Publish an event to the realm
    ///
    /// You can publish 0 or more positional arguments and/or a struct representings Key-Value pairs that have the [Encodable](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/trait.Encodable.html) trait (can be derived from RustcEncodable, see rustc_serialize [docs](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html#rust-type-based-encoding-and-decoding) for more details).
    ///
    /// # Examples
    ///
    /// To send an empty publish event...
    ///
    /// ```rust
    /// use rump::message::WampType;
    /// # use self::Client;
    /// # let mut session = Client::new("", "").connect().unwrap();
    ///
    /// session.publish("com.example.topic", Vec::new(), WampType::None); 
    /// ```
    ///
    /// To send the example positions arguments (42, "foo") and key word arguments...
    /// 
    /// > This library includes a helper enum [WampType](../message/enum.WampType.html) that makes it easy to encode primitive positional arguments and keyword argument maps.
    ///
    /// ```
    /// # use rump::client::Client;
    /// use rump::message::WampType;
    /// # let mut session = Client::new("", "").connect().unwrap();
    ///
    /// extern crate rustc_serialize;
    /// use rustc_serialize::Encodable;
    ///
    /// #[derive(RustcEncodable)]
    /// struct CustomKwargs {
    ///     key1: String,
    ///     key2: u32
    /// }
    ///
    /// session.publish("com.example.topic", vec![WampType::i32(42), WampType::String("foo".to_string())], CustomKwargs {key1: "hello".to_string(), key2: 19});
    /// ```
    ///
    pub fn publish<A, K>(&self, topic: &str, args: Vec<A>, kwargs: K) 
    where A: Encodable, K: Encodable {
        let msg = EventPublish {
            message_type: MessageType::PUBLISH,
            id: new_event_id(),
            topic: topic.to_string(),
            options: Options::Empty, // TODO: Make this an empty field, or with actual options
            args: args,
            kwargs: kwargs,
        };

        self.sender.send(&msg);
    }

    pub fn subscribe<F>(&self, topic: &str, callback: F) 
        where F: 'static + Send + Fn(&Payload) {
            let callback = Box::new(callback);
            let topic = topic.to_string();
            let msg = EventSubscribe::new(topic.clone()); 
            {
                let mut pending = self.pending_subscriptions.lock().unwrap();
                pending.insert(msg.get_id(), (topic.clone(), callback));
            }
            self.sender.send(&msg);
        }
}

impl Client {
    pub fn new(url: &str, realm: &str) -> Self {
        Client {url: String::from(url), realm: String::from(realm)}
    }

    pub fn connect(&self) -> WampResult<Session<WebSocket>> {
        println!("starting...");

        let serializer = Serializer::json();
        let msg_serializer = serializer.clone();
        let pending_subscriptions =  Arc::new(Mutex::new(HashMap::new())); 
        let subscriptions = Arc::new(Mutex::new((HashMap::new(), HashMap::new())));

        let msg_pending_subcriptions = pending_subscriptions.clone();
        let msg_subscriptions = subscriptions.clone();

        let on_message = move |message: Message| {
            if let websocket::message::Type::Text = message.opcode {
                //TODO: Handle unwrap more gracefully
                let payload = from_utf8(message.payload.borrow()).unwrap();
                println!("Got message {:?}", payload);
                if let Ok(event) = msg_serializer.decode::<WampEvent>(payload)  {
                    match event {
                        WampEvent::Subscribed{event_id, topic_id, .. } => { 
                            let (topic_name, callback) : (String, _) = {
                                let mut pending = msg_pending_subcriptions.lock().unwrap();
                                pending.remove(&event_id).expect("Protocol Violation: Got a SUBSCRIBED response, but no subscription was pending from the user.") 
                            };

                            let (ref mut topic_map, ref mut callback_map) = *msg_subscriptions.lock().unwrap();
                            topic_map.insert(topic_id, topic_name.clone());
                            let mut callbacks : &mut Vec<Box<Fn(&Payload) + Send>> = callback_map.entry(topic_name).or_insert(Vec::new());
                            callbacks.push(callback);
                        },
                        WampEvent::Event {topic_id, ..} => {
                            let cb_payload =  Payload::from_str(payload).unwrap();
                            let (ref topic_map, ref callback_map) = *msg_subscriptions.lock().unwrap();
                            let topic_name = topic_map.get(&topic_id).unwrap();
                            for callback in callback_map.get(topic_name).unwrap() {
                                callback(&cb_payload);
                            }
                        },
                    }
                }

            }
        };

        let transport = try!(WebSocket::connect(self.url.clone(), 
                                                serializer,
                                                on_message));

        let session = Session {
            sender: transport, 
            pending_subscriptions: pending_subscriptions,
            subscriptions: subscriptions,
        };
        try!(session.join(self.realm.clone()));
        Ok(session)
    }
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use client::Client;
    use message::WampType;

#[test]
#[ignore]
    fn client_loop_publish() {
        println!("Starting publish session...");
        let mut session = Client::new("ws://localhost:8080/ws", "realm1").connect().unwrap();
        loop {
            let mut counter = 0;
            session.publish("com.myapp.topic1", vec![WampType::i32(counter), 
                            WampType::String("hello".to_string())],
                            WampType::None);
            counter = counter + 1;
        }
    }

#[test]
#[ignore]
    fn client_loop_subscribe() {
        println!("Starting subscribe session...");
        let mut session = Client::new("ws://localhost:8080/ws", "realm1").connect().unwrap();

        #[derive(Debug, RustcDecodable)]
        struct TestStruct {
            counter: i64,
            word: String,
        }

        sleep(Duration::new(1, 0));

        session.subscribe("com.myapp.topic1", |payload| {
            let (counter, from) : (i64, String) = payload.decode_args().unwrap();   
            let test_struct : TestStruct = payload.decode_kwargs().unwrap();
            println!("got count {:?} from {:?}", counter, from);
            println!("and some kwargs {:?}", test_struct);
        });

        loop {}
    }
}
