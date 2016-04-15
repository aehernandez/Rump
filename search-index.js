var searchIndex = {};
searchIndex['rump'] = {"items":[[0,"","rump","",null,null],[4,"WampError","","",null,null],[13,"InvalidURL","","",0,null],[13,"WebSocketError","","",0,null],[13,"InternalThreadError","","",0,null],[0,"client","","",null,null],[3,"Client","rump::client","",null,null],[12,"url","","",1,null],[12,"realm","","",1,null],[3,"Session","","",null,null],[12,"sender","","",2,null],[12,"state","","",2,null],[4,"SessionState","","",null,null],[13,"NotConnected","","",3,null],[13,"Connected","","",3,null],[11,"join","","",2,{"inputs":[{"name":"session"},{"name":"string"}],"output":{"name":"wampresult"}}],[11,"publish","","",2,{"inputs":[{"name":"session"},{"name":"str"},{"name":"vec"},{"name":"k"}],"output":null}],[11,"new","","",1,{"inputs":[{"name":"client"},{"name":"str"},{"name":"str"}],"output":{"name":"self"}}],[11,"connect","","",1,{"inputs":[{"name":"client"}],"output":{"name":"wampresult"}}],[0,"options","rump","",null,null],[3,"Options","rump::options","",null,null],[12,"id","","",4,null],[3,"Details","","",null,null],[12,"roles","","",5,null],[3,"Caller","","",null,null],[12,"features","","",6,null],[3,"CallerFeatures","","",null,null],[12,"caller_identification","","",7,null],[12,"progressive_call_results","","",7,null],[3,"Callee","","",null,null],[12,"features","","",8,null],[3,"CalleeFeatures","","",null,null],[12,"caller_identification","","",9,null],[12,"pattern_based_registration","","",9,null],[12,"shared_registration","","",9,null],[12,"progressive_call_results","","",9,null],[12,"registration_revocation","","",9,null],[3,"Publisher","","",null,null],[12,"features","","",10,null],[3,"PublisherFeatures","","",null,null],[12,"publisher_identification","","",11,null],[12,"subscriber_blackwhite_listing","","",11,null],[12,"publisher_exclusion","","",11,null],[3,"Subscriber","","",null,null],[12,"features","","",12,null],[3,"SubscriberFeatures","","",null,null],[12,"publisher_identification","","",13,null],[12,"pattern_based_subscription","","",13,null],[12,"subscription_revocation","","",13,null],[3,"Roles","","",null,null],[12,"caller","","",14,null],[12,"callee","","",14,null],[12,"publisher","","",14,null],[12,"subscriber","","",14,null],[11,"encode","","",4,{"inputs":[{"name":"options"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",4,{"inputs":[{"name":"options"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",5,{"inputs":[{"name":"details"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",5,{"inputs":[{"name":"details"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",6,{"inputs":[{"name":"caller"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",6,{"inputs":[{"name":"caller"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",7,{"inputs":[{"name":"callerfeatures"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",7,{"inputs":[{"name":"callerfeatures"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",8,{"inputs":[{"name":"callee"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",8,{"inputs":[{"name":"callee"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",9,{"inputs":[{"name":"calleefeatures"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",9,{"inputs":[{"name":"calleefeatures"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",10,{"inputs":[{"name":"publisher"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",10,{"inputs":[{"name":"publisher"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",11,{"inputs":[{"name":"publisherfeatures"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",11,{"inputs":[{"name":"publisherfeatures"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",12,{"inputs":[{"name":"subscriber"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",12,{"inputs":[{"name":"subscriber"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",13,{"inputs":[{"name":"subscriberfeatures"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",13,{"inputs":[{"name":"subscriberfeatures"},{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",14,{"inputs":[{"name":"roles"},{"name":"__s"}],"output":{"name":"result"}}],[11,"decode","","",14,{"inputs":[{"name":"roles"},{"name":"__d"}],"output":{"name":"result"}}],[11,"new","","",5,{"inputs":[{"name":"details"}],"output":{"name":"self"}}],[0,"transport","rump","",null,null],[3,"Serializer","rump::transport","",null,null],[12,"id","","",15,null],[12,"binary","","true if the data being serialized is binary, false otherwise",15,null],[12,"mode","","an enum referring to the type of serilizer",15,null],[3,"WebSocket","","",null,null],[12,"sender","","",16,null],[12,"serializer","","",16,null],[4,"SocketType","","A type enumerating all the possible underlying socket implementations.",null,null],[13,"WEBSOCKET","","",17,null],[4,"SerializerType","","A type enumerating all possible serialization engines",null,null],[13,"JSON","","JSON can be used for human-readable structured data",18,null],[8,"WampConnector","","",null,null],[10,"connect","","",19,{"inputs":[{"name":"wampconnector"},{"name":"string"},{"name":"serializer"},{"name":"f"}],"output":{"name":"wampresult"}}],[8,"WampSender","","",null,null],[10,"send","","",20,{"inputs":[{"name":"wampsender"},{"name":"t"}],"output":{"name":"wampresult"}}],[11,"new","","",15,{"inputs":[{"name":"serializer"},{"name":"serializertype"}],"output":{"name":"self"}}],[11,"encode","","Serialize an encodable message into one that can be sent over a socket",15,{"inputs":[{"name":"serializer"},{"name":"t"}],"output":{"name":"message"}}],[11,"connect","","",16,{"inputs":[{"name":"websocket"},{"name":"string"},{"name":"serializer"},{"name":"f"}],"output":{"name":"wampresult"}}],[11,"send","","",16,{"inputs":[{"name":"websocket"},{"name":"t"}],"output":{"name":"wampresult"}}],[0,"message","rump","",null,null],[3,"EventMessage","rump::message","",null,null],[12,"message_type","","",21,null],[12,"id","","",21,null],[12,"options","","",21,null],[12,"topic","","",21,null],[12,"args","","",21,null],[12,"kwargs","","",21,null],[3,"EventJoin","","",null,null],[12,"message_type","","",22,null],[12,"realm","","",22,null],[12,"details","","",22,null],[4,"MessageType","","All possible WAMP event types",null,null],[13,"HELLO","","",23,null],[13,"WELCOME","","",23,null],[13,"ABORT","","",23,null],[13,"CHALLENGE","","",23,null],[13,"AUTHENTICATE","","",23,null],[13,"GOODBYE","","",23,null],[13,"HEARTBEAT","","",23,null],[13,"ERROR","","",23,null],[13,"PUBLISH","","",23,null],[13,"PUBLISHED","","",23,null],[13,"SUBSCRIBE","","",23,null],[13,"SUBSCRIBED","","",23,null],[13,"UNSUBSCRIBE","","",23,null],[13,"UNSUBSCRIBED","","",23,null],[13,"EVENT","","",23,null],[13,"CALL","","",23,null],[13,"CANCEL","","",23,null],[13,"RESULT","","",23,null],[13,"REGISTER","","",23,null],[13,"REGISTERED","","",23,null],[13,"UNREGISTER","","",23,null],[13,"UNREGISTERED","","",23,null],[13,"INVOCATION","","",23,null],[13,"INTERRUPT","","",23,null],[13,"YIELD","","",23,null],[4,"WampEncodable","","",null,null],[13,"usize","","",24,null],[13,"u8","","",24,null],[13,"u16","","",24,null],[13,"u32","","",24,null],[13,"u64","","",24,null],[13,"isize","","",24,null],[13,"i8","","",24,null],[13,"i16","","",24,null],[13,"i32","","",24,null],[13,"i64","","",24,null],[13,"String","","",24,null],[13,"f32","","",24,null],[13,"f64","","",24,null],[13,"bool","","",24,null],[13,"char","","",24,null],[13,"Generic","","Specify user-defined type. The given type must also implement the trait\n(rustc_serialize::Encodable](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/trait.Encodable.html). If you wish to use several custom types, create an enum over all your types. ",24,null],[13,"None","","",24,null],[5,"new_event_id","","Generate a new id for a WAMP event",null,{"inputs":[],"output":{"name":"u64"}}],[11,"clone","","",23,{"inputs":[{"name":"messagetype"}],"output":{"name":"messagetype"}}],[11,"encode","","",23,{"inputs":[{"name":"messagetype"},{"name":"s"}],"output":{"name":"result"}}],[11,"encode","","",21,{"inputs":[{"name":"eventmessage"},{"name":"s"}],"output":{"name":"result"}}],[11,"encode","","",22,{"inputs":[{"name":"eventjoin"},{"name":"s"}],"output":{"name":"result"}}],[11,"fmt","","",24,{"inputs":[{"name":"wampencodable"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",24,{"inputs":[{"name":"wampencodable"},{"name":"s"}],"output":{"name":"result"}}],[6,"WampResult","rump","",null,null],[11,"fmt","","",0,{"inputs":[{"name":"wamperror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",0,{"inputs":[{"name":"wamperror"},{"name":"senderror"}],"output":{"name":"wamperror"}}],[11,"from","","",0,{"inputs":[{"name":"wamperror"},{"name":"websocketerror"}],"output":{"name":"wamperror"}}]],"paths":[[4,"WampError"],[3,"Client"],[3,"Session"],[4,"SessionState"],[3,"Options"],[3,"Details"],[3,"Caller"],[3,"CallerFeatures"],[3,"Callee"],[3,"CalleeFeatures"],[3,"Publisher"],[3,"PublisherFeatures"],[3,"Subscriber"],[3,"SubscriberFeatures"],[3,"Roles"],[3,"Serializer"],[3,"WebSocket"],[4,"SocketType"],[4,"SerializerType"],[8,"WampConnector"],[8,"WampSender"],[3,"EventMessage"],[3,"EventJoin"],[4,"MessageType"],[4,"WampEncodable"]]};
initSearch(searchIndex);