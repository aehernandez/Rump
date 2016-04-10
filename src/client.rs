extern crate rustc_serialize;
extern crate websocket;

use transport::{WampSender, WampConnector, WebSocket, SerializerType, Serializer};
use message::{WampEncodable, MessageType, EventMessage, EventJoin, new_event_id};
use options::{Options, Details};
use rustc_serialize::Encodable;
use websocket::Message;

use super::{WampResult};

pub struct Client {
    url: String,
    realm: String,
}

pub struct Session <S: WampSender>{
    sender: S,
}

impl <S: WampSender> Session<S> {
    pub fn join(&mut self, realm: String) -> WampResult<()> {
        let join_msg = EventJoin { 
            message_type: MessageType::HELLO,
            realm: realm.clone(),
            details: Details::new(),
        };

       self.sender.send(&join_msg)
    }

    pub fn publish<A, K>(&mut self, topic: &str, args: Vec<WampEncodable<A>>, kwargs: K) where A: Encodable, K: Encodable {
        let msg = EventMessage {
            message_type: MessageType::PUBLISH,
            id: new_event_id(),
            topic: topic.to_string(),
            options: Options {id: 1}, // other stuff goes here
            args: args,
            kwargs: kwargs,
        };

        self.sender.send(&msg);
    }
}

impl Client {
    fn new(url: &str, realm: &str) -> Self {
        Client {url: String::from(url), realm: String::from(realm)}
    }

    fn connect(&self) -> WampResult<Session<WebSocket>> {
        println!("starting...");
        let on_message = move |message: Message| (println!("{:?}", message));
        let transport = try!(WebSocket::connect(self.url.clone(), 
                                                Serializer::new(SerializerType::JSON),
                                                on_message));
        let mut session = Session {sender: transport};
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
}
