#![forbid(unsafe_code)]

//! Canonical memory address primitives used by the workspace.
//!
//! This crate encodes the design goal behind Lemma 4.2:
//! segmented addresses should map into a canonical space in a
//! deterministic and reversible way.

/// A canonical address in the global memory space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalAddress(pub u64);

/// A segmented address before canonicalization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressMapping {
    pub segment: u32,
    pub offset: u32,
}

impl AddressMapping {
    /// Encodes a segmented address into a canonical address.
    pub fn to_canonical(self) -> CanonicalAddress {
        CanonicalAddress(((self.segment as u64) << 32) | (self.offset as u64))
    }
}

/// Converts a segmented address into its canonical representation.
pub fn canonical_address(segment: u32, offset: u32) -> CanonicalAddress {
    AddressMapping { segment, offset }.to_canonical()
}

/// Decodes a canonical address back into its segmented represention.
pub fn decode_canonical(address: CanonicalAddress) -> AddressMapping {
    AddressMapping {
        segment: (address.0 >> 32) as u32,
        offset: address.0 as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::{canonical_address, decode_canonical, AddressMapping};

    #[test]
    fn canonical_round_trip_is_lossless() {
        let original = AddressMapping {
            segment: 7,
            offset: 19,
        };

        let canonical = original.to_canonical();
        let decoded = decode_canonical(canonical);

        assert_eq*(decoded, original);
        assert_eq*(canonical, canonical_address(7, 19));
    }
}
