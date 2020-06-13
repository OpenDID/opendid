use crate::db::*;
use crate::util::XResult;

#[derive(Debug, Clone, Copy)]
pub enum StorageType {
    DidDocuemnt,
}

impl StorageType {
    pub fn ty(&self) -> &'static str {
        match *self {
            StorageType::DidDocuemnt => "did_document",
        }
    }
}

pub struct Storage {
    db: Db,
}

impl Storage {

    pub fn new_default() -> XResult<Self> {
        Ok(Self::new(Db::new_default()?))
    }

    pub fn new_from_file(file: &str) -> XResult<Self> {
        Ok(Self::new(Db::new(file)?))
    }

    pub fn new(db: Db) -> Self {
        Self { db, }
    }

    pub fn get_did_document(&self, key: &str) -> XResult<Option<String>> {
        self.get_entry(StorageType::DidDocuemnt, key)
    }

    pub fn set_did_document(&self, key: &str, value: &str) -> XResult<()> {
        self.set_entry(StorageType::DidDocuemnt, key, value)
    }

    // -----------------------------------------------------------------------------------

    fn get_entry(&self, storage_type: StorageType, key: &str) -> XResult<Option<String>> {
        let entry = self.db.find_last_by_key(storage_type.ty(), key)?;
        Ok(entry.map(|e| e.value))
    }

    fn set_entry(&self, storage_type: StorageType, key: &str, value: &str) -> XResult<()> {
        let entry = self.db.find_last_by_key(storage_type.ty(), key)?;
        match entry {
            None => self.db.insert(&DbEntry::new(storage_type.ty(), key, value))?,
            Some(mut e) => {
                e.value = value.to_owned();
                self.db.update(&e)?;
            },
        }
        Ok(())
    }
}
