use super::keypair::WalletKeyPair;
use super::identity::Identity;

pub struct Wallet {
    key_pairs: Vec<WalletKeyPair>
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            key_pairs: Vec::new()
        }
    }

    pub fn create_identity(&mut self) -> Identity {
        let keypair = WalletKeyPair::generate();
        let public_key = keypair.public_key_bytes();
        let identity = Identity::from_public_key(&public_key);

        self.key_pairs.push(keypair);

        identity
    }
}