use crate::identity::keypair::KeyPair;

#[derive(Debug)]
pub struct Identity {
    pub name: String,
    pub keypair: KeyPair,
}

impl Identity {
    pub fn new(name: String, keypair: KeyPair) -> Self {
        Self { name, keypair }
    }
}
