//! Binary Merkle tree over BLAKE3 (spec sections 3 and 10).
//!
//! Leaves and inner nodes are hashed in separate BLAKE3 key-derivation
//! domains, so a leaf can never be mistaken for an inner node (second
//! preimage attacks). An odd node is carried up unchanged instead of being
//! duplicated (avoids the duplicate-leaf ambiguity known from Bitcoin,
//! CVE-2012-2459), and the root commits to the number of leaves.
//!
//! M3 recomputes the whole tree for every root. A persistent, incremental
//! tree with proofs for dormant accounts follows in M5.

use toy_crypto::hash::blake3_derive;
use toy_types::Hash;

const LEAF_CONTEXT: &str = "ToyChain 2026-10 state Merkle leaf v1";
const NODE_CONTEXT: &str = "ToyChain 2026-10 state Merkle node v1";
const ROOT_CONTEXT: &str = "ToyChain 2026-10 state Merkle root v1";

/// Hash of one leaf from its canonical bytes.
#[must_use]
pub fn leaf_hash(data: &[u8]) -> Hash {
    blake3_derive(LEAF_CONTEXT, data)
}

fn node_hash(left: &Hash, right: &Hash) -> Hash {
    let mut data = [0u8; 64];
    data[..32].copy_from_slice(left.as_bytes());
    data[32..].copy_from_slice(right.as_bytes());
    blake3_derive(NODE_CONTEXT, &data)
}

/// Root over leaf hashes in the given order.
#[must_use]
pub fn merkle_root(leaves: &[Hash]) -> Hash {
    let mut level: Vec<Hash> = leaves.to_vec();
    while level.len() > 1 {
        level = level
            .chunks(2)
            .map(|pair| match pair {
                [left, right] => node_hash(left, right),
                [single] => *single,
                _ => Hash::default(),
            })
            .collect();
    }
    let top = level.first().copied().unwrap_or_default();
    let count = u64::try_from(leaves.len()).unwrap_or(u64::MAX);
    let mut data = [0u8; 40];
    data[..8].copy_from_slice(&count.to_be_bytes());
    data[8..].copy_from_slice(top.as_bytes());
    blake3_derive(ROOT_CONTEXT, &data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaves(n: u8) -> Vec<Hash> {
        (0..n).map(|i| leaf_hash(&[i])).collect()
    }

    #[test]
    fn root_depends_on_every_leaf_and_order() {
        let base = merkle_root(&leaves(5));
        for i in 0..5 {
            let mut changed = leaves(5);
            changed[i] = leaf_hash(&[99]);
            assert_ne!(merkle_root(&changed), base, "leaf {i}");
        }
        let mut swapped = leaves(5);
        swapped.swap(0, 1);
        assert_ne!(merkle_root(&swapped), base);
    }

    #[test]
    fn odd_leaf_is_not_duplicated() {
        // With duplication, [a, b, c] and [a, b, c, c] would share a root.
        let three = leaves(3);
        let mut four = three.clone();
        four.push(three[2]);
        assert_ne!(merkle_root(&three), merkle_root(&four));
    }

    #[test]
    fn empty_and_single() {
        assert_ne!(merkle_root(&[]), merkle_root(&leaves(1)));
        // A single leaf is not its own root (count is committed).
        assert_ne!(merkle_root(&leaves(1)), leaves(1)[0]);
    }
}
