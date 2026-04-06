#![forbid(unsafe_code)]
//! Zeroos-inspired memory isolation scaffolding.

use ark_std::vec::Vec;

/// Represents a memory region under isolation control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub base: u64,
    pub size: u64,
    pub isolated: bool,
}

/// Minimal memory manager placeholder.
#[derive(Debug, Clone, Default)]
pub struct MemoryManager {
    regions: Vec<Region>,
}

impl MemoryManager {
    pub fn map_region(&mut self, base: u64, size: u64, isolated: bool) {
        self.regions.push(Region {
            base,
            size,
            isolated,
        });
    }

    pub fn is_isolated(&self, base: u64) -> Option<bool> {
        self.regions
            .iter()
            .find(|region| region.base == base)
            .map(|region| region.isolated)
    }

    pub fn regions(&self) -> &[Region] {
        &self.regions
    }
}
