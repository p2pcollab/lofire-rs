//! Overlay

use lofire::brokerstore::BrokerStore;
use lofire::store::*;
use lofire::types::*;
use lofire::utils::now_timestamp;
use lofire_net::types::*;
use serde::{Deserialize, Serialize};
use serde_bare::{from_slice, to_vec};

// TODO: versioning V0
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OverlayMeta {
    pub users: u32,
    pub last_used: Timestamp,
}

pub struct Overlay<'a> {
    /// Overlay ID
    id: OverlayId,
    store: &'a dyn BrokerStore,
}

impl<'a> Overlay<'a> {
    const PREFIX: u8 = b"o"[0];

    // propertie's suffixes
    const SECRET: u8 = b"s"[0];
    const PEER: u8 = b"p"[0];
    const TOPIC: u8 = b"t"[0];
    const META: u8 = b"m"[0];
    const REPO: u8 = b"r"[0];

    const ALL_PROPERTIES: [u8; 5] = [
        Self::SECRET,
        Self::PEER,
        Self::TOPIC,
        Self::META,
        Self::REPO,
    ];

    const SUFFIX_FOR_EXIST_CHECK: u8 = Self::SECRET;

    pub fn open(id: &OverlayId, store: &'a dyn BrokerStore) -> Result<Overlay<'a>, StorageError> {
        let opening = Overlay {
            id: id.clone(),
            store,
        };
        if !opening.exists() {
            return Err(StorageError::NotFound);
        }
        Ok(opening)
    }
    pub fn create(
        id: &OverlayId,
        secret: &SymKey,
        repo: Option<PubKey>,
        store: &'a dyn BrokerStore,
    ) -> Result<Overlay<'a>, StorageError> {
        let acc = Overlay {
            id: id.clone(),
            store,
        };
        if acc.exists() {
            return Err(StorageError::BackendError);
        }
        store.put(
            Self::PREFIX,
            &to_vec(&id)?,
            Some(Self::SECRET),
            to_vec(&secret)?,
        )?;
        if repo.is_some() {
            store.put(
                Self::PREFIX,
                &to_vec(&id)?,
                Some(Self::REPO),
                to_vec(&repo.unwrap())?,
            )?;
            //TODO if failure, should remove the previously added SECRET property
        }
        let meta = OverlayMeta {
            users: 1,
            last_used: now_timestamp(),
        };
        store.put(
            Self::PREFIX,
            &to_vec(&id)?,
            Some(Self::META),
            to_vec(&meta)?,
        )?;
        //TODO if failure, should remove the previously added SECRET and REPO properties
        Ok(acc)
    }
    pub fn exists(&self) -> bool {
        self.store
            .get(
                Self::PREFIX,
                &to_vec(&self.id).unwrap(),
                Some(Self::SUFFIX_FOR_EXIST_CHECK),
            )
            .is_ok()
    }
    pub fn id(&self) -> OverlayId {
        self.id
    }
    pub fn add_peer(&self, peer: &PeerId) -> Result<(), StorageError> {
        if !self.exists() {
            return Err(StorageError::BackendError);
        }
        self.store.put(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::PEER),
            to_vec(peer)?,
        )
    }
    pub fn remove_peer(&self, peer: &PeerId) -> Result<(), StorageError> {
        self.store.del_property_value(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::PEER),
            to_vec(peer)?,
        )
    }

    pub fn has_peer(&self, peer: &PeerId) -> Result<(), StorageError> {
        self.store.has_property_value(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::PEER),
            to_vec(peer)?,
        )
    }

    pub fn add_topic(&self, topic: &TopicId) -> Result<(), StorageError> {
        if !self.exists() {
            return Err(StorageError::BackendError);
        }
        self.store.put(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::TOPIC),
            to_vec(topic)?,
        )
    }
    pub fn remove_topic(&self, topic: &TopicId) -> Result<(), StorageError> {
        self.store.del_property_value(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::TOPIC),
            to_vec(topic)?,
        )
    }

    pub fn has_topic(&self, topic: &TopicId) -> Result<(), StorageError> {
        self.store.has_property_value(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::TOPIC),
            to_vec(topic)?,
        )
    }

    pub fn secret(&self) -> Result<SymKey, StorageError> {
        match self
            .store
            .get(Self::PREFIX, &to_vec(&self.id)?, Some(Self::SECRET))
        {
            Ok(secret) => Ok(from_slice::<SymKey>(&secret)?),
            Err(e) => Err(e),
        }
    }

    pub fn metadata(&self) -> Result<OverlayMeta, StorageError> {
        match self
            .store
            .get(Self::PREFIX, &to_vec(&self.id)?, Some(Self::META))
        {
            Ok(meta) => Ok(from_slice::<OverlayMeta>(&meta)?),
            Err(e) => Err(e),
        }
    }
    pub fn set_metadata(&self, meta: &OverlayMeta) -> Result<(), StorageError> {
        if !self.exists() {
            return Err(StorageError::BackendError);
        }
        self.store.replace(
            Self::PREFIX,
            &to_vec(&self.id)?,
            Some(Self::META),
            to_vec(meta)?,
        )
    }

    pub fn repo(&self) -> Result<PubKey, StorageError> {
        match self
            .store
            .get(Self::PREFIX, &to_vec(&self.id)?, Some(Self::REPO))
        {
            Ok(repo) => Ok(from_slice::<PubKey>(&repo)?),
            Err(e) => Err(e),
        }
    }

    pub fn del(&self) -> Result<(), StorageError> {
        self.store
            .del_all(Self::PREFIX, &to_vec(&self.id)?, &Self::ALL_PROPERTIES)
    }
}
