use crate::identity::identity::Identity;

pub struct Wallet {
    identities: Vec<Identity>,
    active_identity: Option<String>,
}

#[derive(Debug)]
pub enum WalletError {
    IdentityAlreadyExists,
    IdentityNotFound,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            identities: Vec::new(),
            active_identity: None,
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

    pub fn set_active_identity(&mut self, name: &str) -> Result<(), WalletError> {
        if !self.contains_identity(name) {
            return Err(WalletError::IdentityNotFound);
        }

        self.active_identity = Some(name.to_string());
        Ok(())
    }

    pub fn active_identity(&self) -> Option<&Identity> {
        let name = self.active_identity.as_ref()?;

        self.identities.iter().find(|id| &id.name == name)
    }

    pub fn remove_identity(&mut self, name: &str) -> Result<(), WalletError>{
        let index = self
            .identities
            .iter()
            .position(|pos| pos.name == name)
            .ok_or(WalletError::IdentityNotFound)
            ?;

        self.identities.remove(index);

        if self.active_identity.as_deref() == Some(name) {
            self.active_identity = None
        }

        Ok(())
    }
}
