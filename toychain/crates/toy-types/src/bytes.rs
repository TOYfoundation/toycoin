use core::fmt;

fn write_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    f.write_str("0x")?;
    for byte in bytes {
        write!(f, "{byte:02x}")?;
    }
    Ok(())
}

/// A 20-byte account address (Ethereum-compatible, spec section 10).
///
/// Derivation from a public key (last 20 bytes of Keccak-256) and the
/// EIP-55 checksum follow in milestone M2.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address([u8; 20]);

impl Address {
    /// Length in bytes.
    pub const LEN: usize = 20;

    /// Wraps raw address bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    /// Returns the raw bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

impl fmt::Display for Address {
    /// Lower-case hex with `0x` prefix (checksum casing comes with M2).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({self})")
    }
}

/// A 32-byte hash value (BLAKE3 or Keccak-256, depending on use).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Length in bytes.
    pub const LEN: usize = 32;

    /// Wraps raw hash bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for Hash {
    /// Lower-case hex with `0x` prefix.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({self})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_display() {
        let mut bytes = [0u8; 20];
        bytes[0] = 0xab;
        bytes[19] = 0x01;
        assert_eq!(
            Address::from_bytes(bytes).to_string(),
            "0xab00000000000000000000000000000000000001"
        );
        assert_eq!(Hash::default().to_string().len(), 2 + 64);
    }
}
