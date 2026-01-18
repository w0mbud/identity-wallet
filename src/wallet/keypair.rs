use ed25519_dalek::{Keypair, Signature, Signer};
use rand::rngs::OsRng;

pub struct WalletKeyPair {
    keypair: Keypair,
}

impl WalletKeyPair {
    pub fn generate() -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        
        Self { keypair }
    }
    
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
    
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }
}
