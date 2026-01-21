use crate::identity::identity::Identity;

pub struct Wallet {
    identities: Vec<Identity>,
}

#[derive(Debug)]
pub enum WalletError {
    IdentityAlreadyExists,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            identities: Vec::new(),
        }
    }

    pub fn add_identity(&mut self, identity: Identity) -> Result<(), WalletError> {
        if self.contains_identity(&identity.name) {
            return Err(WalletError::IdentityAlreadyExists);
        }

        self.identities.push(identity);
        Ok(())
    }

    pub fn identities(&self) -> &Vec<Identity> {
        &self.identities
    }

    pub fn find_identity(&self, name: &str) -> Option<&Identity> {
        self.identities.iter().find(|identity| identity.name == name)
    }

    pub fn contains_identity(&self, name: &str) -> bool {
        self.identities.iter().any(|identity| identity.name == name)
    }
}
