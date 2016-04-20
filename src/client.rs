extern crate rustc_serialize;
extern crate websocket;

use transport::{WampSender, WampConnector, WebSocket, SerializerType, Serializer};
use message::{WampEncodable, MessageType, EventPublish, EventJoin, EventSubscribe, new_event_id};
use options::{Options, Details};

use rustc_serialize::{Encodable, Decodable};

use websocket::{Message};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use super::{WampResult};

use std::thread::sleep;
use std::time::Duration;

/// A Client defines methods and options for building a Session with a WAMP Router
///
/// # Examples
///
/// To obtain a Session from a client with default settings perform the following:
///
/// ```
/// let session = Client::new(ADDR_OF_ROUTER, REALM_NAME).connect().unwrap();
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
    subscriptions: Arc<Mutex<(HashMap<String, u64>, HashMap<u64, Vec<Box<Fn(EventPublish<(), ()>) + Send>>>)>>,
    state: SessionState, 
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
    /// You can publish 0 or more positional arguments that are of type `WampEncodable`
    /// and/or a struct representings Key-Value pairs.
    ///
    ///  # Examples
    ///
    ///  To send an empty publish event 
    ///
    ///  ```
    ///  session.publish::<(), WampEncodable<()>>("com.example.topic", Vec::new(), WampEncodable::None); 
    ///  ```
    ///
    ///  To send the positions arguments (42, "foo") and key word arguments:
    ///  Note: Your custom type must have the
    ///  [Encodable](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/trait.Encodable.html) trait.
    ///
    /// ```
    /// #[derive(RustcEncodable)]
    /// struct CustomKwargs {
    ///     key1: String 
    ///     key2: u32
    /// }
    ///
    /// session.publish::<(), CustomKwargs>("com.example.topic", vec![WampEncodable::i32(42), WampEncodable::String("foo".to_string())], CustomKwargs {key1: "hello".to_string(), key2: 19});
    /// ```
    ///
    pub fn publish<A, K>(&self, topic: &str, args: Vec<WampEncodable<A>>, kwargs: K) where A: Encodable, K: Encodable {
        let msg = EventPublish {
            message_type: MessageType::PUBLISH,
            id: new_event_id(),
            topic: topic.to_string(),
            options: Options {id: 1}, // TODO: Make this an empty field, or with actual options
            args: args,
            kwargs: kwargs,
        };

        self.sender.send(&msg);
    }

    pub fn subscribe<F>(&self, topic: &str, callback: F) 
        where F: 'static + Send + Fn(EventPublish<(), ()>) {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            let callback = Box::new(callback);
            let topic = topic.to_string();
            let msg = EventSubscribe {
                message_type: MessageType::SUBSCRIBE,
                id: new_event_id(),
                topic: topic.clone(),
                options: Options{id: 1},
            };
            self.sender.send(&msg);
            //subscriptions.entry(topic.clone()).or_insert(Vec::new()).push(callback); 
    }
}

impl Client {
    fn new(url: &str, realm: &str) -> Self {
        Client {url: String::from(url), realm: String::from(realm)}
    }

    fn connect(&self) -> WampResult<Session<WebSocket>> {
        println!("starting...");

        let mut state = SessionState::NotConnected;
        // let mut subscriptions = Arc::new(Mutex::new(HashMap::new()));
        // let known_subscriptions = subscriptions.clone();
        let on_message = |message: Message| {
            if let websocket::message::Type::Text = message.opcode {
                let payload = String::from_utf8(message.payload.into_owned());
                println!("Got message {:?}", payload);

            }
        };

        let transport = try!(WebSocket::connect(self.url.clone(), 
                                                Serializer::new(SerializerType::JSON),
                                                on_message));

        let mut session = Session {sender: transport, state: state, 
            subscriptions: unimplemented!()};
        try!(session.join(self.realm.clone()));
        Ok(session)
    }
}

#[test]
fn client_naive_connect() {
    let mut session = Client::new("ws://localhost:8080/ws", "realm1").connect().unwrap();
    session.publish::<(), WampEncodable<()>>("com.myapp.topic1", 
                                             vec![WampEncodable::i32(5), 
                                             WampEncodable::String("hello".to_string())],
                                             WampEncodable::None);
    sleep(Duration::new(2, 0));
    println!("subscribing...");
    session.subscribe("com.myapp.topic1", |m| println!("{:?}", m));

    loop {}
}
