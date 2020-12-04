use blockchain::Block;

use std::time::Instant;

pub struct Chatblock {
    to: String,
    from: String,
    time: Instant,
    msg: String
}

impl Chatblock {
    pub fn new(to: String, from: String, msg: String) -> Self {
        Chatblock {
            to: to,
            from: from,
            time: Instant::now(),
            msg: msg
        }
    }

    pub fn to(&self) -> String {
        self.to
    }

    pub fn from(&self) -> String {
        self.from
    }
    
    pub fn time(&self) -> Instant {
        self.time
    }

    pub fn msg(&self) -> String {
        self.msg
    }
}