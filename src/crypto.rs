use std::str::FromStr;

use anyhow::Error;
use aes_gcm::{aead::{consts::{U32, U12}, generic_array::GenericArray, rand_core::RngCore}, KeyInit, Nonce};
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::Aead;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::msg("Failed to hash password"))?
        .to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(&password_hash) {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    } else  {
        false
    }
}


// Struct that holds all the information needed to decrypt the secret later
pub struct AesData {
    pub nonce: Nonce<U12>,
    pub salt: SaltString,
    pub ciphertext: Vec<u8>,
}

impl TryFrom<&str> for AesData {
    type Error = Error;

    fn try_from(value: &str) -> Result<AesData, Error> {
        let mut parts = value.split('$');
        let nonce = parts.next().ok_or(Error::msg("Invalid AES data string"))?;
        let salt = parts.next().ok_or(Error::msg("Invalid AES data string"))?;
        let ciphertext = parts.next().ok_or(Error::msg("Invalid AES data string"))?;

        let nonce: [u8; 12] = hex::decode(nonce)?.as_slice().try_into()?;
        let nonce: Nonce<U12> = nonce.into();

        let salt = SaltString::new(salt)
            .map_err(|_| Error::msg("Failed to generate salt"))?;

        let ciphertext = hex::decode(ciphertext)?;

        Ok(AesData { nonce, salt, ciphertext })
    }
}

impl FromStr for AesData {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.try_into()  
    }
}

impl From<AesData> for String {
    fn from(data: AesData) -> String  {
        let nonce = hex::encode(data.nonce.as_slice());
        let salt = data.salt.as_str();
        let ciphertext = hex::encode(data.ciphertext);

        format!("{}${}${}", nonce, salt, ciphertext)
    }
}

impl AesData {
    pub fn encrypt(plaintext: &str, password: &str) -> Result<AesData, Error> {
        // Derive a 256bit key from the password using Argon2
        let salt = SaltString::generate(&mut OsRng);
        let key = derive_key(password, &salt)?;

        // Generate a nonce
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let nonce: Nonce<U12> = nonce.into();

        // Encrypt using the derived key
        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|_| Error::msg("Failed to encrypt"))?;

        Ok(AesData { nonce, salt, ciphertext })
    }

    pub fn decrypt(&self, password: &str) -> Result<String, Error> {
        // Derive the encryption key from password and salt using Argon2
        let AesData { ciphertext, salt, nonce } = self;
        let key = derive_key(password, &salt)?;

        let cipher = Aes256Gcm::new(&key);
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_slice())
            .map_err(|_| Error::msg("Failed to decrypt. Did you mistype your password?"))?;

        let plaintext = String::from_utf8(plaintext)?;
        Ok(plaintext)
    }
}

fn derive_key(password: &str, salt: &SaltString) -> Result<GenericArray<u8, U32>, Error> {
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::msg("Failed to derive key"))?
        .hash.ok_or(Error::msg("Failed to derive key"))?;

    let key: &GenericArray<u8, U32> = hash
        .as_bytes()
        .try_into()?;

    Ok(key.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() -> Result<(), Error> {
        let password = "supersecret";
        let hash = hash_password(password)?;

        assert!(verify_password(password, &hash));
        Ok(())
    }

    #[test]
    fn test_encryption() -> Result<(), Error> {
        let plaintext = "this is super secret";
        let password = "password";
        let aes_data: AesData = AesData::encrypt(plaintext, password)?;

        assert_eq!(plaintext, aes_data.decrypt(password)?);
        Ok(())
    }
}
