use block::{DynBlock, Block};
use ring::digest::Digest;
use block_cryptography_rust::hashing::sha256_hash;
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
            msg: msg,
            time: Instant::now
        }
    }

    pub fn to(&self) -> &String {
        &self.to
    }

    pub fn from(&self) -> &String {
        &self.from
    }

    pub fn time(&self) -> &String {
        &format!("{:?}", self.time)
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }
}

impl Block for Chatblock {
    fn hash(&self) -> Digest {
        sha256_hash(format!("{}{}{}{:?}", &self.to, &self.from, &self.msg, &self.time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_block_test() {
        let block = Chatblock::new("Jake".to_string(), "Ryan".to_string(), "Hi".to_string());

        assert_eq!(block.to(), "Jake".to_string());
        assert_eq!(block.from(), "Ryan".to_string());
        assert_eq!(block.msg(), "Hi".to_string());
    }
}