#![cfg(not(target_family = "wasm"))]
//! Types that are used in RPC.
//! Compile this file only for non-wasm ISA because of the thndermint-rpc
//! dependency on socket2 (issue https://github.com/rust-lang/socket2/issues/268)

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use anoma_proof_of_stake::types::Slashes;
use jsonpath_lib as jsonpath;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "ABCI"))]
use tendermint::abci::Path as AbciPath;
#[cfg(not(feature = "ABCI"))]
use tendermint_rpc::error::Error as TError;
#[cfg(feature = "ABCI")]
use tendermint_rpc_abci::error::Error as TError;
#[cfg(feature = "ABCI")]
use tendermint_stable::abci::Path as AbciPath;
use thiserror::Error;

use super::address;
use super::token::Amount;
use crate::types::address::Address;
use crate::types::hash::{self, Hash};
use crate::types::storage::{self, BlockHeight};

const DRY_RUN_TX_PATH: &str = "dry_run_tx";
const EPOCH_PATH: &str = "epoch";
const VALUE_PREFIX: &str = "value";
const PREFIX_PREFIX: &str = "prefix";
const HAS_KEY_PREFIX: &str = "has_key";
const ACCEPTED: &str = "accepted";
const APPLIED: &str = "applied";

/// Tendermint Event types
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TendermintEventType {
    /// Accepted is ony supported by ABCI++
    Accepted,
    /// Applied is always supported
    Applied,
}

impl Display for TendermintEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "{}", ACCEPTED),
            Self::Applied => write!(f, "{}", APPLIED),
        }
    }
}

impl From<TendermintEventType> for &str {
    fn from(item: TendermintEventType) -> Self {
        match item {
            TendermintEventType::Accepted => ACCEPTED,
            TendermintEventType::Applied => APPLIED,
        }
    }
}

impl TryFrom<&str> for TendermintEventType {
    type Error = EventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            #[cfg(not(feature = "ABCI"))]
            ACCEPTED => Ok(Self::Accepted), // Accepted is ony supported by
            // ABCI++
            APPLIED => Ok(Self::Applied),
            _ => Err(EventError(value.to_owned())),
        }
    }
}

/// The error generated by an invalid tendermint event
#[derive(Debug, Error)]
#[error("Unsupported Tendermint event {0}")]
pub struct EventError(String);

/// The result of a tx query.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxQueryResult {
    /// The tendermint response for tx
    pub response: TxResponse,
    /// The tendermint type of the tx
    pub event_type: TendermintEventType,
}

impl Display for TxQueryResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Tx {}", self.event_type)?;
        writeln!(f, "{}", self.response)?;

        Ok(())
    }
}

/// The result of a bond query.
#[derive(Clone, Copy, Debug, Default)]
pub struct BondQueryResult {
    /// Total bonds
    pub bonds: Amount,
    /// Active bonds
    pub active: Amount,
    /// Total unbonds
    pub unbonds: Amount,
    /// Whithdrawable unbonds
    pub withdrawable: Amount,
}

impl Display for BondQueryResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Total bonds: {}", self.bonds)?;
        writeln!(f, "Active bonds: {}", self.active)?;
        writeln!(f, "Total unbonds: {}", self.unbonds)?;
        writeln!(f, "Withdrawable unbonds: {}", self.withdrawable)?;
        Ok(())
    }
}

/// The result of a slash query.
#[derive(Clone, Debug, Default)]
pub struct SlashQueryResult(HashMap<Address, Slashes>);

impl Deref for SlashQueryResult {
    type Target = HashMap<Address, Slashes>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SlashQueryResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for SlashQueryResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "No slashes were found")?;
            return Ok(());
        }

        writeln!(f, "Slashes:")?;

        for (validator, slashes) in self.iter() {
            println!("Slashes for validator {}", validator);
            for slash in slashes {
                println!(
                    "{:4}Epoch: {}, block height: {}, type: {}, rate: {}",
                    "",
                    slash.epoch,
                    slash.block_height,
                    slash.r#type,
                    slash.rate
                );
            }
        }

        Ok(())
    }
}

impl AsRef<HashMap<Address, Slashes>> for SlashQueryResult {
    fn as_ref(&self) -> &HashMap<Address, Slashes> {
        &self.0
    }
}

/// The result of a balance query. First Address is the owner one,
/// second Address is the token one.
#[derive(Clone, Debug, Default)]
pub struct BalanceQueryResult(HashMap<Address, HashMap<Address, Amount>>);

impl BalanceQueryResult {
    /// Insert token balance for the given owner.
    pub fn insert(&mut self, owner: Address, token: Address, balance: Amount) {
        let balances = self.entry(owner).or_insert_with(HashMap::new);
        balances.insert(token, balance);
    }

    /// Get the token balance for the provided owner.
    pub fn get_balance(
        &self,
        owner: &Address,
        token: &Address,
    ) -> Option<Amount> {
        match self.get(owner) {
            Some(inner) => inner.get(token).cloned(),
            None => None,
        }
    }
}

impl Deref for BalanceQueryResult {
    type Target = HashMap<Address, HashMap<Address, Amount>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BalanceQueryResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for BalanceQueryResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "No balances were found")?;
            return Ok(());
        }

        writeln!(f, "Balances:")?;

        for (owner, tokens) in self.iter() {
            writeln!(f, "{:4}Owner: {}", "", owner)?;
            for (token, balance) in tokens {
                let token_str = match address::tokens().get(token) {
                    Some(t) => t.to_string(),
                    None => token.to_string(),
                };
                writeln!(f, "{:8} {}: {}", "", token_str, balance)?;
            }
        }

        Ok(())
    }
}

impl AsRef<HashMap<Address, HashMap<Address, Amount>>> for BalanceQueryResult {
    fn as_ref(&self) -> &HashMap<Address, HashMap<Address, Amount>> {
        &self.0
    }
}

/// RPC query path
#[derive(Debug, Clone)]
pub enum Path {
    /// Dry run a transaction
    DryRunTx,
    /// Epoch of the last committed block
    Epoch,
    /// Read a storage value with exact storage key
    Value(storage::Key),
    /// Read a range of storage values with a matching key prefix
    Prefix(storage::Key),
    /// Check if the given storage key exists
    HasKey(storage::Key),
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::DryRunTx => write!(f, "{}", DRY_RUN_TX_PATH),
            Path::Epoch => write!(f, "{}", EPOCH_PATH),
            Path::Value(storage_key) => {
                write!(f, "{}/{}", VALUE_PREFIX, storage_key)
            }
            Path::Prefix(storage_key) => {
                write!(f, "{}/{}", PREFIX_PREFIX, storage_key)
            }
            Path::HasKey(storage_key) => {
                write!(f, "{}/{}", HAS_KEY_PREFIX, storage_key)
            }
        }
    }
}

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.to_lowercase();
        match path.as_str() {
            DRY_RUN_TX_PATH => Ok(Self::DryRunTx),
            EPOCH_PATH => Ok(Self::Epoch),
            _ => match path.split_once('/') {
                Some((VALUE_PREFIX, storage_key)) => {
                    let key = storage::Key::parse(storage_key)
                        .map_err(PathParseError::InvalidStorageKey)?;
                    Ok(Self::Value(key))
                }
                Some((PREFIX_PREFIX, storage_key)) => {
                    let key = storage::Key::parse(storage_key)
                        .map_err(PathParseError::InvalidStorageKey)?;
                    Ok(Self::Prefix(key))
                }
                Some((HAS_KEY_PREFIX, storage_key)) => {
                    let key = storage::Key::parse(storage_key)
                        .map_err(PathParseError::InvalidStorageKey)?;
                    Ok(Self::HasKey(key))
                }
                _ => Err(PathParseError::InvalidPath(s.to_string())),
            },
        }
    }
}

impl From<Path> for AbciPath {
    fn from(path: Path) -> Self {
        let path = path.to_string();
        // TODO: update in tendermint-rs to allow to construct this from owned
        // string. It's what `from_str` does anyway
        AbciPath::from_str(&path).unwrap()
    }
}

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum PathParseError {
    #[error("Unrecognized query path: {0}")]
    InvalidPath(String),
    #[error("Invalid storage key: {0}")]
    InvalidStorageKey(storage::Error),
}

/// The tendermint response for a tx
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TxResponse {
    /// Tx info
    pub info: String,
    /// TX log
    pub log: String,
    /// Height of the block containing tx
    pub height: BlockHeight,
    /// Hash of the tx
    pub hash: Hash,
    /// Exit code of tx
    pub code: u8,
    /// Gas used for tx
    pub gas_used: u64,
    /// Accounts initialized by tx
    pub initialized_accounts: Vec<Address>,
}

impl Display for TxResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Hash: {}", self.hash)?;
        writeln!(f, "Code: {}", self.code)?;
        writeln!(f, "Info: {}", self.info)?;
        writeln!(f, "Log: {}", self.log)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Gas used: {}", self.gas_used)?;

        if !self.initialized_accounts.is_empty() {
            writeln!(f, "Initialized accounts:")?;
            for account in &self.initialized_accounts {
                writeln!(f, "{:4}", account)?;
            }
        }

        Ok(())
    }
}

impl TxResponse {
    /// Retrieve the response for the given tx hash from the provided json
    /// serialized result.
    ///
    /// # Panics
    ///
    /// Will panic if the requested hash is not found or if the json schema
    /// differs from the expected one.
    #[cfg(feature = "ABCI")]
    pub fn find_tx<T>(
        json_response: &serde_json::Value,
        tx_hash: T,
    ) -> Result<Self, QueryError>
    where
        T: AsRef<str>,
    {
        let mut selector = jsonpath::selector(json_response);

        let mut response = TxResponse::default();

        let mut index = 0usize;
        let hash_ref = tx_hash.as_ref();

        response.hash = loop {
            let hash =
                selector(&format!("$.events.['applied.hash'][{}]", index))?[0];
            let hash_str: String = serde_json::from_value(hash.to_owned())?;
            if hash_str.as_str() == hash_ref {
                break Hash::try_from(hash_ref)?;
            }
            index += 1;
        };

        response.info = {
            let value =
                selector(&format!("$.events.['applied.info'][{}]", index))?[0];
            serde_json::from_value(value.to_owned())?
        };

        response.code = {
            let value =
                selector(&format!("$.events.['applied.code'][{}]", index))?[0];
            u8::from_str(value.as_str().unwrap())?
        };

        response.gas_used = {
            let value =
                selector(&format!("$.events.['applied.gas_used'][{}]", index))?
                    [0];
            u64::from_str(value.as_str().unwrap())?
        };

        response.log = {
            let value =
                selector(&format!("$.events.['applied.log'][{}]", index))?[0];
            serde_json::from_value(value.to_owned())?
        };

        response.height = {
            let value =
                selector(&format!("$.events.['applied.height'][{}]", index))?
                    [0];
            BlockHeight(u64::from_str(value.as_str().unwrap())?)
        };

        response.initialized_accounts = {
            let values = selector(&format!(
                "$.events.['applied.initialized_accounts'][{}]",
                index
            ))?;
            match values.get(0) {
                // In a response, the initialized accounts are encoded as e.g.:
                // ```
                // "applied.initialized_accounts": Array([
                //   String(
                //     "[\"atest1...\"]",
                //   ),
                // ]),
                // ...
                // So we need to decode the vec from the array inside the string
                Some(v) => {
                    serde_json::from_str(v.to_owned().as_str().unwrap())?
                }
                None => vec![],
            }
        };

        Ok(response)
    }

    /// Retrieve the response for the given tx hash from the provided json
    /// serialized result.
    ///
    /// # Panics
    ///
    /// Will panic if the json schema differs from the expected one.
    #[cfg(not(feature = "ABCI"))]
    pub fn find_tx<E, T>(
        json_response: &serde_json::Value,
        event_type: E,
        tx_hash: T,
    ) -> Result<Self, QueryError>
    where
        E: AsRef<str>,
        T: AsRef<str> + Into<String>,
    {
        let mut response = TxResponse::default();
        let mut selector = jsonpath::selector(json_response);
        let evt_key = TendermintEventType::try_from(event_type.as_ref())?;

        response.hash = Hash::try_from(tx_hash.as_ref())?;

        let event = selector(&format!(
            "$.events.[?(@.type=='{}' && @.attributes.[?(@.key=='hash' && \
             @.value=='{}')])]",
            evt_key,
            tx_hash.into()
        ))?;

        for obj in event[0]
            .as_object()
            .unwrap()
            .get("attributes")
            .unwrap()
            .as_array()
            .unwrap()
        {
            let obj = obj.as_object().unwrap();
            match obj.get("key").unwrap().as_str().unwrap() {
                "info" => {
                    response.info = serde_json::from_value(
                        obj.get("value").unwrap().to_owned(),
                    )?
                }
                "code" => {
                    response.code = u8::from_str(
                        obj.get("value").unwrap().as_str().unwrap(),
                    )?
                }
                "gas_used" => {
                    response.gas_used = u64::from_str(
                        obj.get("value").unwrap().as_str().unwrap(),
                    )?
                }
                "log" => {
                    response.log = serde_json::from_value(
                        obj.get("value").unwrap().to_owned(),
                    )?
                }
                "height" => {
                    response.height = BlockHeight(u64::from_str(
                        obj.get("value").unwrap().as_str().unwrap(),
                    )?)
                }
                "initialized_accounts" => {
                    response.initialized_accounts =
                        match obj.get("value").unwrap().as_str() {
                            // In a response, the initialized accounts are
                            // encoded as e.g.: ```
                            // "applied.initialized_accounts": Array([
                            //   String(
                            //     "[\"atest1...\"]",
                            //   ),
                            // ]),
                            // ...
                            // So we need to decode the vec from the array
                            // inside the string
                            Some(v) => serde_json::from_str(v)?,
                            None => vec![],
                        };
                }
                _ => (),
            }
        }

        Ok(response)
    }
}

/// The error generated by an RPC query
#[derive(Debug, Error)]
pub enum QueryError {
    /// General ABCI error
    #[error("Abci query failed: {0}")]
    ABCIQueryError(TError),
    /// Invalid conversion from String
    #[error("Error while casting value from String {0}")]
    ConversionError(#[from] std::num::ParseIntError),
    /// Decoding error
    #[error("Error decoding the value: {0}")]
    Decoding(#[from] std::io::Error),
    /// Bad query format
    #[error("Error in the query {0} (error code {1})")]
    Format(String, u32),
    /// Hash decoding error from bytes
    #[error("{0}")]
    FromHexError(#[from] hash::Error),
    /// Block not found
    #[error("Unable to find a block applying the given transaction hash {0}")]
    BlockNotFound(Hash),
    /// Event not found
    #[error(
        "Unable to find the event corresponding to the given transaction hash \
         {0}"
    )]
    EventNotFound(Hash),
    /// Json error
    #[error("Error with json path")]
    JsonError(#[from] jsonpath::JsonPathError),
    /// Negative voting power delta
    #[error("The sum voting power deltas shouldn't be negative")]
    NegativeVotingPowerDeltas(#[from] std::num::TryFromIntError),
    /// serde_json error
    #[error("Couldn't load from serde value: {0}")]
    SerdeError(#[from] serde_json::Error),
    /// Unset validator set
    #[error("Validator set should always be set")]
    UnsetValidatorSet,
    /// Unset voting power
    #[error("Total voting power should always be set")]
    UnsetVotingPower,
    /// Unsupported tendermint event
    #[error("{0}")]
    UnsupportedTendermintEvent(#[from] EventError),
    /// Transaction not found
    #[error("Unable to query for transaction with given hash")]
    TxNotFound(#[from] TError),
}
