pub mod key_chain {
    use crate::utils::utils::file_system;
    use orion::kdf;
    use orion::pwhash;
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{Error, ErrorKind};

    pub fn get_key_salt(id: &str) -> Result<[u8; 16], Error> {
        let mut buffer: [u8; 16] = [0; 16];
        let complete_path = file_system::generate_salt_path(id).unwrap();

        let mut f = File::open(complete_path)?;
        f.read(&mut buffer)?;
        Ok(buffer)
    }

    pub fn get_pass_hash(id: &str) -> Result<String, Error> {
        let complete_path = file_system::generate_pass_path(id).unwrap();

        Ok(fs::read_to_string(complete_path)?)
    }

    pub fn valid_auth(id: &str, pass: &str) -> Result<(), orion::errors::UnknownCryptoError> {
        if let Ok(hash) = get_pass_hash(id) {
            let stored_hash = pwhash::PasswordHash::from_encoded(&hash)?;
            let password = pwhash::Password::from_slice(pass.as_ref())?;
            return Ok(pwhash::hash_password_verify(&stored_hash, &password)?);
        }

        Err(orion::errors::UnknownCryptoError)
    }

    pub fn create_user(id: &str, password: &str) -> Result<(), Error> {
        create_key_salt(id)?;
        create_password(id, password)?;

        Ok(())
    }

    pub fn create_password(id: &str, password: &str) -> Result<(), Error> {
        let pass_path = file_system::generate_pass_path(id).unwrap();
        file_system::generate_key_dir(id)?;
        let mut pass_file = File::create(pass_path)?;

        if let Ok(password) = pwhash::Password::from_slice(password.as_ref()) {
            if let Ok(hash) = pwhash::hash_password(&password, 3, 1 << 16) {
                pass_file.write_all(hash.unprotected_as_encoded().as_ref())?;

                return Ok(());
            }
        }

        Err(Error::new(ErrorKind::NotFound, "CryptoError"))
    }

    pub fn create_key_salt(id: &str) -> Result<(), Error> {
        let salt_path = file_system::generate_salt_path(id).unwrap();
        file_system::generate_key_dir(id)?;

        let mut salt_file = File::create(salt_path)?;
        let salt = kdf::Salt::default();
        salt_file.write_all(salt.as_ref())?;
        Ok(())
    }

    pub fn get_env_var(var_name: &str) -> Result<String, Error> {
        match env::var(var_name) {
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                var_name.to_owned() + " env variable Not Found",
            )),
            Ok(key_dir) => Ok(key_dir),
        }
    }

    #[allow(dead_code)]
    fn find_id_filename(id: &str) -> Result<String, Error> {
        match get_key_ids() {
            None => Err(Error::new(ErrorKind::NotFound, "Could not get key ids")),
            Some(ids) => {
                let mut it = ids.iter();
                match it.position(|key| key == id) {
                    Some(found) => Ok(file_system::generate_path(&[&ids[found], &".txt"])),
                    None => Err(Error::new(ErrorKind::NotFound, "Could not find id")),
                }
            }
        }
    }

    pub fn id_exists(id: &str) -> bool {
        if let Some(ids) = get_key_ids() {
            return ids.contains(&String::from(id));
        }
        return false;
    }

    pub fn get_key_names() -> Result<Vec<String>, Error> {
        match env::var("DATABASE_KEYS_DIR") {
            Ok(ids_dir) => {
                let ids = file_system::read_dir_to_string(ids_dir)?;
                Ok(ids)
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "DATABASE_IDS_DIR env variable Not Found",
            )),
        }
    }

    pub fn get_key_ids() -> Option<Vec<String>> {
        match get_key_names() {
            Ok(key_filenames) => {
                let keys = key_filenames
                    .into_iter()
                    .map(|filename| String::from(&filename))
                    .collect();
                Some(keys)
            }
            Err(_) => None,
        }
    }
}
