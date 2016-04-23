use rustc_serialize::{json, Encodable, Encoder};
use message::WampType;

/// Represents different Options that can be sent wth a WAMP event 
#[derive(RustcDecodable, Debug, Clone, PartialEq)]
pub enum Options {
    // TODO: // different options
    Empty,
}

impl Encodable for Options {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match self {
           &Options::Empty => s.emit_map(0, |s| Ok(())) 
        }
    }
}

/// Represents advanced features this WAMP Client implements
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Details { 
    roles: Roles 
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct Caller {
    features: CallerFeatures 
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct CallerFeatures {
    caller_identification: bool,
    progressive_call_results: bool,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct Callee {
    features: CalleeFeatures
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct CalleeFeatures {
    caller_identification: bool,
    pattern_based_registration: bool,
    shared_registration: bool,
    progressive_call_results: bool,
    registration_revocation: bool,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct Publisher {
    features: PublisherFeatures,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct PublisherFeatures {
    publisher_identification: bool,
    subscriber_blackwhite_listing: bool,
    publisher_exclusion: bool
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct Subscriber {
    features: SubscriberFeatures,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct SubscriberFeatures {
    publisher_identification: bool,
    pattern_based_subscription: bool,
    subscription_revocation: bool
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
struct Roles {
    caller: Caller,
    callee: Callee,
    publisher: Publisher,
    subscriber: Subscriber,
}

impl Details {
    pub fn new() -> Self {
        Details { 
            roles: Roles {
                caller: Caller {
                    features: CallerFeatures {
                        caller_identification: false,
                        progressive_call_results: false
                    }
                },
                callee: Callee {
                    features: CalleeFeatures {
                        caller_identification: false,
                        pattern_based_registration: false,
                        shared_registration: false,
                        progressive_call_results: false,
                        registration_revocation: false
                    }
                },
                publisher: Publisher {
                    features: PublisherFeatures {
                        publisher_identification: false,
                        subscriber_blackwhite_listing: false,
                        publisher_exclusion: false,
                    }
                },
                subscriber: Subscriber {
                    features: SubscriberFeatures {
                       publisher_identification: false,
                       pattern_based_subscription: false,
                       subscription_revocation: false,
                    }
                }
            },
        }

    }
}


