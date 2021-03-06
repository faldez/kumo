use crate::db::{MangaDatabase, UserDatabase};
use crate::extension::ExtensionBus;

pub struct GlobalContext {
    pub userdb: UserDatabase,
    pub secret: String,
    pub mangadb: MangaDatabase,
    pub extensions: ExtensionBus,
}

impl GlobalContext {
    pub fn new(userdb: UserDatabase, mangadb: MangaDatabase, secret: String, extensions: ExtensionBus) -> Self {
        Self {
            userdb,
            mangadb,
            secret,
            extensions,
        }
    }
}
