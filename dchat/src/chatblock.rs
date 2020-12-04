use blockchain::Block;
use block_cryptography_rust::hashing::sha256_hash;
use block_cryptography_rust::signing::{RSAKeyPair, RSASignature, sign_data};

use ring::digest::Digest;
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

impl Block for Chatblock {
    fn hash(&self) -> Digest {
        sha256_hash(format!("{}{}{}", self.to, self.from, self.msg).as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chatblock_test() {
        let to = "Ryan".to_string();
        let from = "Jake".to_string();
        let msg = "Hello".to_string();

        let block = Chatblock::new(to, from, msg);

        assert_eq!(block.to(), to);
        assert_eq!(block.from(), from);
        assert_eq!(block.msg(), msg);
    }
}