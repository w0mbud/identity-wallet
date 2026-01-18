use base64::{Engine as _, engine::general_purpose};

pub struct Identity {
    pub did: String,
    pub public_key: Vec<u8>
}

impl Identity {
    pub fn from_public_key(public_key: &[u8]) -> Self {
        let did = Self::derive_did(public_key);

        Self {
            did,
            public_key: public_key.to_vec(),
        }
    }

    fn derive_did(public_key: &[u8]) -> String {
        let encoded = general_purpose::STANDARD.encode(public_key);
        format!("did:key:{}", encoded)
    }
}
