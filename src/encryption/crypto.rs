pub mod crypto {
    use crate::database::keychain::key_chain;
    use orion::aead;
    use orion::kdf;
    pub fn encrypt_str(
        user: &str,
        message: &str,
        password: &str,
    ) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
        let pass_salt = recover_pass_salt(user, password)?;
        let derived_key = kdf::derive_key(&pass_salt.0, &pass_salt.1, 3, 1 << 16, 32)?;

        aead::seal(&derived_key, message.as_bytes())
    }

    pub fn decrypt_str(
        user: &str,
        ciphertext: &str,
        password: &str,
    ) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
        let pass_salt = recover_pass_salt(user, password)?;
        let secret_key = kdf::derive_key(&pass_salt.0, &pass_salt.1, 3, 1 << 16, 32)?;
        aead::open(&secret_key, &ciphertext.as_bytes())
    }

    pub fn recover_pass_salt(
        user: &str,
        password: &str,
    ) -> Result<(kdf::Password, kdf::Salt), orion::errors::UnknownCryptoError> {
        let pass = kdf::Password::from_slice(password.as_bytes())?;
        let mut salt = kdf::Salt::default();
        match key_chain::get_key_salt(user) {
            Ok(salt_str) => Ok(salt = kdf::Salt::from_slice(&salt_str)?),
            Err(_) => Err(orion::errors::UnknownCryptoError),
        }?;

        Ok((pass, salt))
    }
}
