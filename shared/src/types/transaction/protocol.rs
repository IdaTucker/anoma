/// Types for sending and verifying txs
/// used in Anoma protocols
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::types::address::Address;

/// A tx data type to initialize a new validator account and its staking reward
/// account.
#[derive(
    Debug,
    Clone,
    PartialEq,
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
)]
pub struct UpdateDkgSessionKey {
    /// The storage key of the validators public DKG session key
    pub address: Address,
    /// The serialization of the new public key associated with the validator
    pub dkg_public_key: Vec<u8>,
}

#[cfg(feature = "ferveo-tpke")]
mod protocol_txs {
    use std::io::{ErrorKind, Write};
    use std::path::Path;

    use borsh::{BorshDeserialize, BorshSerialize};
    use ferveo::dkg::pv::Message;
    use serde_json;

    use super::*;
    use crate::proto::Tx;
    use crate::types::key::*;
    use crate::types::transaction::{EllipticCurve, TxError, TxType};

    const TX_NEW_DKG_KP_WASM: &str = "tx_update_dkg_session_keypair.wasm";

    #[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
    /// Txs sent by validators as part of internal protocols
    pub struct ProtocolTx {
        /// we require ProtocolTxs be signed
        pub pk: common::PublicKey,
        /// The type of protocol message being sent
        pub tx: ProtocolTxType,
    }

    impl ProtocolTx {
        /// Validate the signature of a protocol tx
        pub fn validate_sig(
            &self,
            signed_hash: [u8; 32],
            sig: &common::Signature,
        ) -> Result<(), TxError> {
            common::SigScheme::verify_signature(&self.pk, &signed_hash, sig)
                .map_err(|err| {
                    TxError::SigError(format!(
                        "ProtocolTx signature verification failed: {}",
                        err
                    ))
                })
        }
    }

    #[derive(Clone, Debug)]
    #[allow(clippy::large_enum_variant)]
    /// Types of protocol messages to be sent
    pub enum ProtocolTxType {
        /// Messages to be given to the DKG state machine
        DKG(Message<EllipticCurve>),
        /// Tx requesting a new DKG session keypair
        NewDkgKeypair(Tx),
        /// Aggregation of Ethereum state changes
        /// voted on by validators in last block
        EthereumStateUpdate(Tx),
    }

    impl ProtocolTxType {
        /// Sign a ProtocolTxType and wrap it up in a normal Tx
        pub fn sign(
            self,
            pk: &common::PublicKey,
            signing_key: &common::SecretKey,
        ) -> Tx {
            Tx::new(
                vec![],
                Some(
                    TxType::Protocol(ProtocolTx {
                        pk: pk.clone(),
                        tx: self,
                    })
                    .try_to_vec()
                    .expect("Could not serialize ProtocolTx"),
                ),
            )
            .sign(signing_key)
        }

        /// Create a new tx requesting a new DKG session keypair
        pub fn request_new_dkg_keypair<'a, F>(
            data: UpdateDkgSessionKey,
            signing_key: &common::SecretKey,
            wasm_dir: &'a Path,
            wasm_loader: F,
        ) -> Self
        where
            F: FnOnce(&'a str, &'static str) -> Vec<u8>,
        {
            let code = wasm_loader(
                wasm_dir
                    .to_str()
                    .expect("Converting path to string should not fail"),
                TX_NEW_DKG_KP_WASM,
            );
            Self::NewDkgKeypair(
                Tx::new(
                    code,
                    Some(
                        data.try_to_vec()
                            .expect("Serializing request should not fail"),
                    ),
                )
                .sign(signing_key),
            )
        }
    }

    impl borsh::ser::BorshSerialize for ProtocolTxType {
        fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
            let blob = match &self {
                ProtocolTxType::NewDkgKeypair(tx) => (1u8, tx.try_to_vec()?),
                ProtocolTxType::DKG(msg) => (
                    0u8,
                    serde_json::to_string(&msg)
                        .map_err(|err| {
                            std::io::Error::new(ErrorKind::InvalidData, err)
                        })?
                        .as_bytes()
                        .to_owned(),
                ),
                ProtocolTxType::EthereumStateUpdate(tx) => {
                    (2u8, tx.try_to_vec()?)
                }
            };
            BorshSerialize::serialize(&blob, writer)
        }
    }

    impl borsh::de::BorshDeserialize for ProtocolTxType {
        fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
            let (variant, blob): (u8, Vec<u8>) =
                BorshDeserialize::deserialize(buf)?;
            match variant {
                0 => {
                    let json = String::from_utf8(blob).map_err(|err| {
                        std::io::Error::new(ErrorKind::InvalidData, err)
                    })?;
                    Ok(ProtocolTxType::DKG(
                        serde_json::from_str(&json).map_err(|err| {
                            std::io::Error::new(ErrorKind::InvalidData, err)
                        })?,
                    ))
                }
                1 => Ok(ProtocolTxType::NewDkgKeypair(
                    BorshDeserialize::deserialize(&mut blob.as_ref())?,
                )),
                2 => Ok(ProtocolTxType::EthereumStateUpdate(
                    BorshDeserialize::deserialize(&mut blob.as_ref())?,
                )),
                _ => Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid enum variant",
                )),
            }
        }
    }

    impl From<Message<EllipticCurve>> for ProtocolTxType {
        fn from(msg: Message<EllipticCurve>) -> ProtocolTxType {
            ProtocolTxType::DKG(msg)
        }
    }
}

#[cfg(feature = "ferveo-tpke")]
pub use protocol_txs::*;