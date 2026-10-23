//! ToyWork: memory-hard proof of work with Argon2id (spec sections 9
//! and 10), wrapping the RustCrypto `argon2` crate.
//!
//! A ToyWork hash is Argon2id (version 0x13) with 16 MiB of memory, one
//! pass and one lane over a 32-byte input. The caller binds the input to
//! an address, an epoch and the epoch seed (see `toy-tx`), so tickets can
//! neither be computed in advance nor reused for someone else.
//!
//! The memory (16 MiB) is allocated once per [`ToyWorkHasher`] and reused,
//! so checking many tickets in a row does not allocate again.

use argon2::{Algorithm, Argon2, Block, Params, Version};
use toy_types::Hash;
use toy_types::params::{TOYWORK_ITERATIONS, TOYWORK_MEMORY_KIB, TOYWORK_PARALLELISM};

use crate::CryptoError;

/// Fixed Argon2 salt of ToyWork. The salt only separates ToyWork hashes
/// from Argon2 uses elsewhere (keystore, M8); the per-ticket uniqueness
/// comes from the input.
pub const TOYWORK_SALT: &[u8; 16] = b"ToyChain-ToyWork";

/// Output length of a ToyWork hash in bytes.
const OUTPUT_LEN: usize = 32;

/// Reusable ToyWork hasher holding the 16 MiB Argon2 work area.
pub struct ToyWorkHasher {
    argon2: Argon2<'static>,
    memory: Vec<Block>,
}

impl ToyWorkHasher {
    /// Creates a hasher with the protocol parameters and allocates its
    /// memory.
    ///
    /// # Errors
    /// [`CryptoError::WorkFailed`] if the parameters are rejected (not
    /// expected for the fixed protocol values).
    pub fn new() -> Result<Self, CryptoError> {
        let params = Params::new(
            TOYWORK_MEMORY_KIB,
            TOYWORK_ITERATIONS,
            TOYWORK_PARALLELISM,
            Some(OUTPUT_LEN),
        )
        .map_err(|_| CryptoError::WorkFailed)?;
        let blocks = params.block_count();
        Ok(Self {
            argon2: Argon2::new(Algorithm::Argon2id, Version::V0x13, params),
            memory: vec![Block::default(); blocks],
        })
    }

    /// Computes the ToyWork hash of a 32-byte input.
    ///
    /// # Errors
    /// [`CryptoError::WorkFailed`] if Argon2 reports an error (not
    /// expected: input, salt and output lengths are fixed and valid).
    pub fn hash(&mut self, input: &Hash) -> Result<Hash, CryptoError> {
        let mut out = [0u8; OUTPUT_LEN];
        self.argon2
            .hash_password_into_with_memory(
                input.as_bytes(),
                TOYWORK_SALT,
                &mut out,
                &mut self.memory,
            )
            .map_err(|_| CryptoError::WorkFailed)?;
        Ok(Hash::from_bytes(out))
    }
}

impl core::fmt::Debug for ToyWorkHasher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ToyWorkHasher({} KiB)", self.memory.len())
    }
}

/// Reads the first 16 bytes of a ToyWork hash as a big-endian number.
/// A ticket is valid if this value is below the difficulty threshold.
#[must_use]
pub fn work_value(hash: &Hash) -> u128 {
    let mut head = [0u8; 16];
    if let Some(src) = hash.as_bytes().get(..16) {
        head.copy_from_slice(src);
    }
    u128::from_be_bytes(head)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Official Argon2id test vector (RFC 9106, section 5.3), taken from the
    /// reference implementation P-H-C/phc-winner-argon2, file
    /// `kats/argon2id` (see `tests/vectors/README.md`). It checks that the
    /// wrapped crate computes Argon2id version 0x13 exactly as specified;
    /// ToyWork then only fixes the parameters.
    #[test]
    fn rfc9106_argon2id_vector() {
        let params = argon2::ParamsBuilder::new()
            .m_cost(32)
            .t_cost(3)
            .p_cost(4)
            .output_len(32)
            .data(argon2::AssociatedData::new(&[0x04; 12]).unwrap())
            .build()
            .unwrap();
        let argon2 =
            Argon2::new_with_secret(&[0x03; 8], Algorithm::Argon2id, Version::V0x13, params)
                .unwrap();
        let mut out = [0u8; 32];
        let mut memory = vec![Block::default(); 32];
        argon2
            .hash_password_into_with_memory(&[0x01; 32], &[0x02; 16], &mut out, &mut memory)
            .unwrap();
        assert_eq!(
            hex(&out),
            "0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659"
        );
    }

    #[test]
    fn toywork_uses_16_mib_and_is_deterministic() {
        let mut hasher = ToyWorkHasher::new().unwrap();
        assert_eq!(hasher.memory.len(), 16 * 1024);
        let a = hasher.hash(&Hash::from_bytes([7; 32])).unwrap();
        let b = hasher.hash(&Hash::from_bytes([7; 32])).unwrap();
        let c = hasher.hash(&Hash::from_bytes([8; 32])).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
        // A second hasher (fresh memory) gives the same result.
        let mut other = ToyWorkHasher::new().unwrap();
        assert_eq!(other.hash(&Hash::from_bytes([7; 32])).unwrap(), a);
    }

    #[test]
    fn toywork_regression_value() {
        // Pins the exact ToyWork function (parameters, salt, version). If
        // this changes, every ticket ever made becomes invalid: a
        // consensus change that needs a protocol update.
        let mut hasher = ToyWorkHasher::new().unwrap();
        let hash = hasher.hash(&Hash::from_bytes([0; 32])).unwrap();
        assert_eq!(hash.to_string(), REGRESSION);
    }

    /// Cross-checked on 2026-10-20 with the C reference library libargon2
    /// (Python `argon2-cffi`, `hash_secret_raw` with type ID, version 19,
    /// m = 16384, t = 1, p = 1, salt `ToyChain-ToyWork`, 32 zero bytes).
    const REGRESSION: &str = "0xcde78d8cbcee8a3e780e2971be7556d34a24f46f8126370b3fefc4786ec6b478";

    #[test]
    fn work_value_reads_big_endian_prefix() {
        let mut bytes = [0xffu8; 32];
        bytes[0] = 0;
        bytes[15] = 1;
        assert_eq!(
            work_value(&Hash::from_bytes(bytes)),
            0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ff01
        );
    }
}
