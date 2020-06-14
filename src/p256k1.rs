use secp256k1::{ Secp256k1, Message, Signature, SecretKey, PublicKey, };
use crate::util::XResult;

pub struct P256k1SecretKey {
    secret_key: SecretKey,
}

impl P256k1SecretKey {

    pub fn from_hex(secret_key_bytes: &str) -> XResult<Self> {
        Self::from_slice(&hex::decode(secret_key_bytes)?[..])
    }

    pub fn from_slice(secret_key_bytes: &[u8]) -> XResult<Self> {
        let secret_key = SecretKey::from_slice(&secret_key_bytes[..])?;
        Ok(Self {
            secret_key,
        })
    }

    pub fn sign(&self, message: &Message) -> Signature {
        let secp = Secp256k1::new();
        // let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        let sig = secp.sign(&message, &self.secret_key);
        // println!("{}", sig);
        sig
    }
}

pub struct P256k1PublicKey {
    public_key: PublicKey,
}

impl P256k1PublicKey {

    pub fn from_secret_key(p256k1_secret_key: &P256k1SecretKey) -> Self {
        let secp = Secp256k1::new(); // should new ?
        Self {
            public_key: PublicKey::from_secret_key(&secp, &p256k1_secret_key.secret_key),
        }
    }

    pub fn as_string(&self) -> String {
        hex::encode(&self.public_key.serialize_uncompressed()[..])
    }
}
