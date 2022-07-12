use anoma::types::address::Address;

/// Vp imports and functions.
pub mod vp {
    pub use anoma::types::key::*;

    use super::*;
    use crate::imports::vp::*;

    /// Get the public key associated with the given address. Panics if not
    /// found.
    pub fn get(
        ctx: &Ctx,
        owner: &Address,
    ) -> EnvResult<Option<common::PublicKey>> {
        let key = pk_key(owner);
        ctx.read_pre(&key)
    }
}
