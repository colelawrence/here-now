#![allow(unused)]

use base64::Engine;
pub use hpke;
use rand::{rngs::StdRng, SeedableRng};

use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Deserializable, Kem as KemTrait, OpModeR, OpModeS, Serializable,
};
use serde::de::Visitor;

pub use self::net::LocalKeys;

// These are the only algorithms we're gonna use for this example
pub type Kem = X25519HkdfSha256;
pub type Aead = ChaCha20Poly1305;
pub type Kdf = HkdfSha384;

pub type PrivateKey = <Kem as KemTrait>::PrivateKey;
pub type PublicKey = <Kem as KemTrait>::PublicKey;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum PublicKeyKind {
    #[serde(serialize_with = "serde_ser_key", deserialize_with = "serde_des_key")]
    X25519HkdfSha256(<Kem as KemTrait>::PublicKey),
}

impl std::fmt::Debug for PublicKeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X25519HkdfSha256(arg0) => f
                .debug_tuple("X25519HkdfSha256")
                .field(&ser_key(arg0))
                .finish(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum PrivateKeyKind {
    #[serde(serialize_with = "serde_ser_key", deserialize_with = "serde_des_key")]
    X25519HkdfSha256(<Kem as KemTrait>::PrivateKey),
}

const BASE_64_URL_ENGINE: base64::engine::GeneralPurpose =
    base64::engine::general_purpose::URL_SAFE_NO_PAD;

fn serde_ser_key<S: serde::Serializer, K: Serializable>(
    key: &K,
    ser: S,
) -> Result<S::Ok, S::Error> {
    ser.collect_str(&ser_key(key))
}

struct KeyVisitor<K> {
    _marker: std::marker::PhantomData<K>,
}
impl<K> KeyVisitor<K> {
    fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
impl<'de, K: Deserializable> Visitor<'de> for KeyVisitor<K> {
    type Value = K;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a base64 url key string")
    }
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut seq = seq;
        let mut bytes = Vec::new();
        while let Some(item) = seq.next_element::<u8>()? {
            bytes.push(item);
        }
        K::from_bytes(&bytes).map_err(|k| <A::Error as serde::de::Error>::custom(k))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        des_key(v)
    }
}
fn serde_des_key<'de, D: serde::Deserializer<'de>, K: Deserializable>(
    des: D,
) -> Result<K, D::Error> {
    des.deserialize_string(KeyVisitor::<K>::new())
}
fn ser_key<K: Serializable>(key: &K) -> String {
    BASE_64_URL_ENGINE.encode(key.to_bytes())
}

fn des_key<K: Deserializable, E: serde::de::Error>(base64_key: &str) -> Result<K, E> {
    K::from_bytes(
        &BASE_64_URL_ENGINE
            .decode(base64_key)
            .map_err(|e| E::custom(e))?,
    )
    .map_err(|k| E::custom(k))
}

impl std::fmt::Debug for PrivateKeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X25519HkdfSha256(arg0) => f
                .debug_tuple("X25519HkdfSha256")
                .field(&"(private)")
                .finish(),
        }
    }
}

/// Initialize a fresh keypair
pub fn init() -> LocalKeys {
    let mut csprng = StdRng::from_entropy();
    let (privatek, publ) = Kem::gen_keypair(&mut csprng);
    LocalKeys::new(
        PrivateKeyKind::X25519HkdfSha256(privatek),
        PublicKeyKind::X25519HkdfSha256(publ),
    )
}

pub mod net {
    use std::{
        any::type_name,
        marker::PhantomData,
        time::{Duration, Instant, SystemTime},
    };

    use super::*;
    use anyhow::Context;

    const INFO_STR: &[u8] = b"hn net session";

    #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
    pub struct LocalKeys(super::PrivateKeyKind, super::PublicKeyKind);

    impl LocalKeys {
        pub fn new(sk: super::PrivateKeyKind, pk: super::PublicKeyKind) -> Self {
            Self(sk, pk)
        }
        pub fn public_key(&self) -> &super::PublicKeyKind {
            &self.1
        }
        pub fn send<T: EncryptableMessage>(
            &self,
            msg: T,
            rx_pk: &super::PublicKeyKind,
        ) -> anyhow::Result<WireMessage> {
            encrypt_msg(
                &MessageHeader {
                    nonce: Nonce::new(),
                    txpub: self.1.clone(),
                },
                msg,
                rx_pk,
                &self.0,
                &self.1,
            )
        }
        pub fn recv<T: serde::de::DeserializeOwned>(
            &self,
            msg: &WireMessage,
        ) -> anyhow::Result<VerifiedMessage<T>> {
            decrypt_msg(&self.0, msg).with_context(|| format!("decrypting {}", type_name::<T>()))
        }
    }

    pub struct RawWireResult<E>(Vec<u8>, PhantomData<E>);

    impl<E> RawWireResult<E> {
        pub fn from_ok<S: serde::Serialize>(input: S) -> Self {
            let ok: Result<S, ()> = Ok(input);
            Self(
                pot::to_vec(&ok).expect("could not serialize message!"),
                PhantomData,
            )
        }
        pub fn from_err(input: E) -> Self
        where
            E: serde::Serialize,
        {
            let err: Result<(), E> = Err(input);
            Self(
                pot::to_vec(&err).expect("could not serialize message!"),
                PhantomData,
            )
        }
    }

    pub trait EncryptableMessage {
        fn into_bytes(self) -> Vec<u8>;
    }

    impl<S: serde::Serialize> EncryptableMessage for &S {
        fn into_bytes(self) -> Vec<u8> {
            pot::to_vec(self).expect("could not serialize message!")
        }
    }
    impl<E> EncryptableMessage for RawWireResult<E> {
        fn into_bytes(self) -> Vec<u8> {
            self.0
        }
    }

    // Given a message and associated data, returns an encapsulated key, ciphertext, and tag. The
    // ciphertext is encrypted with the shared AEAD context
    fn encrypt_msg<T: EncryptableMessage>(
        header: &MessageHeader,
        msg: T,
        rx_pk: &PublicKeyKind,
        tx_sk: &PrivateKeyKind,
        tx_pk: &PublicKeyKind,
    ) -> anyhow::Result<WireMessage> {
        let mut csprng = StdRng::from_entropy();
        let associated_data = pot::to_vec(header).context("serializing header")?;

        let (encapped_key_bytes, ciphertext, tag_bytes) = match rx_pk {
            PublicKeyKind::X25519HkdfSha256(server_pk) => {
                // Encapsulate a key and use the resulting shared secret to encrypt a message. The AEAD context
                // is what you use to encrypt.
                let (encapped_key, mut sender_ctx) = hpke::setup_sender::<Aead, Kdf, Kem, _>(
                    &OpModeS::Auth((
                        match tx_sk {
                            PrivateKeyKind::X25519HkdfSha256(tx_sk) => tx_sk.clone(),
                        },
                        match tx_pk {
                            PublicKeyKind::X25519HkdfSha256(tx_pk) => tx_pk.clone(),
                        },
                    )),
                    server_pk,
                    INFO_STR,
                    &mut csprng,
                )
                .context("invalid server pubkey!")?;

                // On success, seal_in_place_detached() will encrypt the plaintext in place
                let mut msg_copy = msg.into_bytes();
                let tag = sender_ctx
                    .seal_in_place_detached(&mut msg_copy, &associated_data)
                    .context("encryption failed!")?;

                // Rename for clarity
                let ciphertext = msg_copy;

                // Now imagine we send everything over the wire, so we have to serialize it
                let encapped_key_bytes = encapped_key.to_bytes().to_vec();
                let tag_bytes = tag.to_bytes().to_vec();

                (encapped_key_bytes, ciphertext, tag_bytes)
            }
        };

        Ok(WireMessage {
            associated_data,
            encapped_key_bytes,
            ciphertext,
            tag_bytes,
        })
    }

    // Returns the decrypted client message
    fn decrypt_msg<T: serde::de::DeserializeOwned>(
        server_sk: &PrivateKeyKind,
        wire_message: &WireMessage,
    ) -> anyhow::Result<VerifiedMessage<T>> {
        let header = pot::from_slice::<MessageHeader>(&wire_message.associated_data)
            .context("parsing header")?;

        let tag = AeadTag::<Aead>::from_bytes(&wire_message.tag_bytes)
            .context("could not deserialize AEAD tag!")?;
        let encapped_key =
            <Kem as KemTrait>::EncappedKey::from_bytes(&wire_message.encapped_key_bytes)
                .context("could not deserialize the encapsulated pubkey!")?;

        let plaintext = match server_sk {
            PrivateKeyKind::X25519HkdfSha256(server_sk) => {
                // Decapsulate and derive the shared secret. This creates a shared AEAD context.
                let mut receiver_ctx = hpke::setup_receiver::<Aead, Kdf, Kem>(
                    &OpModeR::Auth(match &header.txpub {
                        PublicKeyKind::X25519HkdfSha256(tx_pk) => tx_pk.clone(),
                    }),
                    &server_sk,
                    &encapped_key,
                    INFO_STR,
                )
                .context("failed to set up receiver!")?;
                // On success, open_in_place_detached() will decrypt the ciphertext in place
                let mut ciphertext_copy = wire_message.ciphertext.to_vec();
                receiver_ctx
                    .open_in_place_detached(
                        &mut ciphertext_copy,
                        &wire_message.associated_data,
                        &tag,
                    )
                    .context("invalid ciphertext!")?;

                ciphertext_copy
            }
        };

        let data = pot::from_slice(&plaintext).with_context(|| {
            format!(
                "deserializing plaintext into message type: {:?}",
                String::from_utf8_lossy(&plaintext)
            )
        })?;

        Ok(VerifiedMessage { header, data })
    }

    /// Structureless 
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct WireMessage {
        /// Associated data would be the place where we should put the sender's identity
        /// like the device_id, a "nonce", a client's public key, and
        /// perhaps any other routing information necessary (but we only send directly to the server, so maybe it's not necessary to have routing info)
        #[serde(rename = "a")]
        associated_data: Vec<u8>,
        #[serde(rename = "e")]
        encapped_key_bytes: Vec<u8>,
        #[serde(rename = "c")]
        ciphertext: Vec<u8>,
        #[serde(rename = "t")]
        tag_bytes: Vec<u8>,
    }

    impl WireMessage {
        pub fn to_bytes(&self) -> Vec<u8> {
            pot::to_vec(self).expect("could not serialize wire message!")
        }
        pub fn from_bytes(serialized: &[u8]) -> anyhow::Result<Self> {
            pot::from_slice(serialized).with_context(|| {
                format!(
                    "deserializing WireMessage from pot bytes: {:?}",
                    String::from_utf8_lossy(serialized)
                )
            })
        }
    }

    pub struct VerifiedMessage<T> {
        header: MessageHeader,
        data: T,
    }

    impl<T> VerifiedMessage<T> {
        pub fn sender(&self) -> &super::PublicKeyKind {
            &self.header.txpub
        }
        pub fn nonce(&self) -> &Nonce {
            &self.header.nonce
        }
        pub fn data(&self) -> &T {
            &self.data
        }
        pub fn into_data(self) -> T {
            self.data
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct MessageHeader {
        txpub: PublicKeyKind,
        nonce: Nonce,
        // assoc: Vec<u8>,
    }

    #[test]
    fn test_enc_dec() {
        // Set up the server
        let server_keys = init();
        let client_keys = init();

        // The message to be encrypted
        let msg = (b"Kat Branchman".to_vec(), 12u32);

        // Let the client send a message to the server using the server's pubkey
        let wire_message = client_keys
            .send(&msg, server_keys.public_key())
            .expect("msg encrypt failed!");

        // Now let the server decrypt the message. The to_bytes() calls returned a GenericArray, so we
        // have to convert them to slices before sending them
        let decrypted_msg: VerifiedMessage<(Vec<u8>, u32)> = server_keys
            .recv(&wire_message)
            .expect("msg decrypt failed!");

        // Make sure everything decrypted correctly
        assert_eq!(decrypted_msg.data(), &msg);
        assert_eq!(
            serde_json::to_string(decrypted_msg.sender()).unwrap(),
            serde_json::to_string(client_keys.public_key()).unwrap(),
        );

        println!(
            "MESSAGE SUCCESSFULLY SENT AND RECEIVED {:?}",
            decrypted_msg.nonce()
        );
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    #[serde(transparent)]
    pub struct Nonce(String);

    impl std::fmt::Display for Nonce {
        fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(&mut f)
        }
    }

    impl Nonce {
        pub fn new() -> Self {
            use rand::RngCore;
            let mut nonce_bytes = [0u8; 24];
            let prefix = (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32)
                .to_be_bytes();
            nonce_bytes[..4].copy_from_slice(&prefix);
            let mut csprng = StdRng::from_entropy();
            csprng.fill_bytes(&mut nonce_bytes[4..]);
            Nonce(BASE_64_URL_ENGINE.encode(nonce_bytes))
        }
    }
}

#[test]
fn test() {
    let v = serde_json::to_string(&init()).unwrap();
    eprintln!("init = {v}");
    let v = serde_json::from_str::<(PrivateKeyKind, PublicKeyKind)>(&v).unwrap();
    eprintln!("back = {v:?}");
}
