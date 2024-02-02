/// Decrypts arbitrary data.
///
/// # Arguments
/// * `encrypted_data` - The [`EncryptedData`] to be decrypted.
/// * `sender_public_key_hex` - The sender's public key in hex format.
/// * `recipient_private_key_hex` - The private agreement key of the recipient in hex format.
///
/// # Returns
/// A array of [`u8`] representing the decrypted data.
pub fn decrypt_data(
    encrypted_data: impl EncryptedData,
    sender_public_key_hex: &str,
    recipient_private_key_hex: &str,
) {
    todo!()
}

/// Decrypts an encrypted message.
///
/// # Arguments
/// * `encrypted_base64` - Encrypted data in base64 format.
///
/// # Returns
/// A [`String`] containing the decrypted content or an std::error::Error
/// if decryption failed.
pub fn decrypt_aes(_encrypted_base64: &str, _key: &str) -> String {
    todo!()
}

/// Hashes a string slice into a hex string.
///
/// # Arguments
/// * `input` - A string slice holding the text to be hashed.
///
/// # Returns
/// A [`String`] containing the hashed text in hex format.
pub fn hash_sha256(_input: &str) -> String {
    todo!()
}

pub trait EncryptedData {}
