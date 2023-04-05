use thiserror::Error;

/// Errors that can occur in the nostr-proto crate
#[derive(Error, Debug)]
pub enum Error {
    /// Assertion failed
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    /// Bad Encrypted Message
    #[error("Bad Encrypted Message")]
    BadEncryptedMessage,

    /// Base64 error
    #[error("Base64 Decoding Error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// Bech32 error
    #[error("Bech32 Error: {0}")]
    Bech32(#[from] bech32::Error),

    /// Encryption/Decryption Error
    #[error("Encryption/Decryption Error")]
    Encryption,

    /// Bech32 error
    #[error("Wrong Bech32 Kind: Expected {0} found {0}")]
    WrongBech32(String, String),

    /// Signature error
    #[error("ECDSA Signature Error: {0}")]
    Signature(#[from] k256::ecdsa::Error),

    /// Event is in the future
    #[error("Event is in the future")]
    EventInFuture,

    /// Formatting error
    #[error("Formatting Error: {0}")]
    Fmt(#[from] std::fmt::Error),

    /// A hash mismatch verification error
    #[error("Hash Mismatch")]
    HashMismatch,

    /// Hex string decoding error
    #[error("Hex Decode Error: {0}")]
    HexDecode(#[from] hex::FromHexError),

    /// Invalid encrypted private key
    #[error("Invalid Encrypted Private Key")]
    InvalidEncryptedPrivateKey,

    /// Invalid event Id
    #[error("Invalid event Id")]
    InvalidId,

    /// Invalid event Id Prefix
    #[error("Invalid event Id Prefix")]
    InvalidIdPrefix,

    /// Invalid digest length
    #[error("Invalid digest length")]
    InvalidLength(#[from] hmac::digest::InvalidLength),

    /// Invalid Profile
    #[error("Invalid Profile")]
    InvalidProfile,

    /// Invalid public key
    #[error("Invalid Public Key")]
    InvalidPublicKey,

    /// Invalid public key prefix
    #[error("Invalid Public Key Prefix")]
    InvalidPublicKeyPrefix,

    /// Invalid URL
    #[error("Invalid URL: \"{0}\"")]
    InvalidUrl(#[from] url::ParseError),

    /// Invalid URL Host
    #[error("Invalid URL Host: \"{0}\"")]
    InvalidUrlHost(String),

    /// Invalid URL Scheme
    #[error("Invalid URL Scheme: \"{0}\"")]
    InvalidUrlScheme(String),

    /// Missing URL Authority
    #[error("Missing URL Authority")]
    InvalidUrlMissingAuthority,

    /// Pad error
    #[error("Encryption/Decryption padding error")]
    Pad(#[from] inout::PadError),

    /// Parse integer error
    #[error("Parse integer error")]
    ParseInt(#[from] std::num::ParseIntError),

    /// Scrypt error
    #[error("Scrypt invalid output length")]
    Scrypt,

    /// Serialization error
    #[error("JSON (de)serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// Try from slice error
    #[error("Try From Slice error: {0}")]
    Slice(#[from] std::array::TryFromSliceError),

    /// Time error
    #[error("System Time Error: {0}")]
    Time(#[from] std::time::SystemTimeError),

    /// Unknown event kind
    #[error("Unknown event kind = {0}")]
    UnknownEventKind(u64),

    /// Unknown Key Security
    #[error("Unknown key security = {0}")]
    UnknownKeySecurity(u8),

    /// Unpad error
    #[error("Decryption error: {0}")]
    Unpad(#[from] aes::cipher::block_padding::UnpadError),

    /// Url Error
    #[error("Not a valid nostr relay url: {0}")]
    Url(String),

    /// UTF-8 error
    #[error("UTF-8 Error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Wrong event kind
    #[error("Wrong event kind")]
    WrongEventKind,

    /// Wrong length hex string
    #[error("Wrong length hex string")]
    WrongLengthHexString,

    /// Wrong Decryption Password
    #[error("Wrong decryption password")]
    WrongDecryptionPassword,
}
