var searchIndex = {};
searchIndex['rump'] = {"items":[[0,"","rump","A [WAMP](http://wamp-proto.org/) Client built on pure Rust.",null,null],[3,"Payload","","A struct representing the payload that's received from a WAMP event.",null,null],[4,"WampType","","All types that can be sent to/from a WAMP Router\nUsed to publish hetereogenous arguments",null,null],[13,"usize","","",0,null],[13,"u8","","",0,null],[13,"u16","","",0,null],[13,"u32","","",0,null],[13,"u64","","",0,null],[13,"isize","","",0,null],[13,"i8","","",0,null],[13,"i16","","",0,null],[13,"i32","","",0,null],[13,"i64","","",0,null],[13,"String","","",0,null],[13,"f32","","",0,null],[13,"f64","","",0,null],[13,"bool","","",0,null],[13,"char","","",0,null],[13,"Vec","","",0,null],[13,"Map","","",0,null],[13,"None","","Used to send an empty struct or keymap value \"{}\"",0,null],[4,"WampError","","",null,null],[13,"InvalidURL","","",1,null],[13,"WebSocketError","","",1,null],[13,"InternalThreadError","","",1,null],[13,"ProtocolError","","",1,null],[13,"DecodeError","","",1,null],[0,"client","","",null,null],[3,"Client","rump::client","A Client defines methods and options for building a Session with a WAMP Router",null,null],[3,"Session","","A Session represents a valid WAMP Session with a Router.\nYou can obtain a `Session` from a `Client`",null,null],[11,"join","","Connects to a WAMP Router in a realm without authentication",2,{"inputs":[{"name":"session"},{"name":"string"}],"output":{"name":"wampresult"}}],[11,"publish","","Publish an event to the realm",2,{"inputs":[{"name":"session"},{"name":"str"},{"name":"vec"},{"name":"k"}],"output":null}],[11,"subscribe","","",2,{"inputs":[{"name":"session"},{"name":"str"},{"name":"f"}],"output":null}],[11,"new","","",3,{"inputs":[{"name":"client"},{"name":"str"},{"name":"str"}],"output":{"name":"self"}}],[11,"connect","","",3,{"inputs":[{"name":"client"}],"output":{"name":"wampresult"}}],[11,"fmt","rump","",4,{"inputs":[{"name":"payload"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","Parse args and kwargs from a WAMP Message that is known to have a payload",4,{"inputs":[{"name":"payload"},{"name":"str"}],"output":{"name":"wampresult"}}],[11,"decode_args","","Extract positional arguments from the payload.",4,{"inputs":[{"name":"payload"}],"output":{"name":"wampresult"}}],[11,"decode_kwargs","","Extract keyword arguments from the payload.",4,{"inputs":[{"name":"payload"}],"output":{"name":"wampresult"}}],[11,"has_args","","",4,{"inputs":[{"name":"payload"}],"output":{"name":"bool"}}],[11,"has_kwargs","","",4,{"inputs":[{"name":"payload"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"wamptype"},{"name":"wamptype"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"wamptype"},{"name":"wamptype"}],"output":{"name":"bool"}}],[11,"clone","","",0,{"inputs":[{"name":"wamptype"}],"output":{"name":"wamptype"}}],[11,"fmt","","",0,{"inputs":[{"name":"wamptype"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",0,{"inputs":[{"name":"wamptype"},{"name":"s"}],"output":{"name":"result"}}],[6,"WampResult","","",null,null],[11,"fmt","","",1,{"inputs":[{"name":"wamperror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",1,{"inputs":[{"name":"wamperror"},{"name":"senderror"}],"output":{"name":"wamperror"}}],[11,"from","","",1,{"inputs":[{"name":"wamperror"},{"name":"websocketerror"}],"output":{"name":"wamperror"}}]],"paths":[[4,"WampType"],[4,"WampError"],[3,"Session"],[3,"Client"],[3,"Payload"]]};
initSearch(searchIndex);