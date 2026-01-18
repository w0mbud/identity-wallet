#[derive(Debug)]
pub struct KeyPair {
    pub public_key: String,
    pub private_key: String,
}

impl KeyPair {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self { public_key, private_key }
    }
}
