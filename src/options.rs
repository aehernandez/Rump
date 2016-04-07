use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Options {
    // TODO: // different options
    pub id: u32,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Details { 
    roles: Roles 
}

#[derive(RustcDecodable, RustcEncodable)]
struct Caller {
    features: CallerFeatures 
}

#[derive(RustcDecodable, RustcEncodable)]
struct CallerFeatures {
    caller_identification: bool,
    progressive_call_results: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Callee {
    features: CalleeFeatures
}

#[derive(RustcDecodable, RustcEncodable)]
struct CalleeFeatures {
    caller_identification: bool,
    pattern_based_registration: bool,
    shared_registration: bool,
    progressive_call_results: bool,
    registration_revocation: bool,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Publisher {
    features: PublisherFeatures,
}

#[derive(RustcDecodable, RustcEncodable)]
struct PublisherFeatures {
    publisher_identification: bool,
    subscriber_blackwhite_listing: bool,
    publisher_exclusion: bool
}

#[derive(RustcDecodable, RustcEncodable)]
struct Subscriber {
    features: SubscriberFeatures,
}

#[derive(RustcDecodable, RustcEncodable)]
struct SubscriberFeatures {
    publisher_identification: bool,
    pattern_based_subscription: bool,
    subscription_revocation: bool
}

#[derive(RustcDecodable, RustcEncodable)]
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


