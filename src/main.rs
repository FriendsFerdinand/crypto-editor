use std::env;

pub mod utils {
    pub mod list_tools;
    pub mod utils;
}
pub use utils::*;

pub mod encryption {
    pub mod crypto;
    pub use crypto::crypto::*;
}

pub mod database {
    pub mod database_handler;
    pub mod keychain;
    pub use database_handler::*;
    pub use keychain::*;
}
#[path = "crypto_editor.rs"]
mod crypto_editor;
pub use crypto_editor::CryptoEditor;

pub mod menu {
    pub mod menu;
}
pub use menu::menu::menu::*;

fn main() {
    env::set_var("DATABASE_IDS_DIR", "./src/database/logs");
    env::set_var("DATABASE_KEYS_DIR", "./src/database/keys");

    run();
}
