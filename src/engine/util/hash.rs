/// A utility for deterministic, consistent hashing across the engine.
/// Currently powered by `seahash` for speed and determinism.
pub struct Hasher {
    state: u64,
}

impl Hasher {
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Hashes a u64 value and updates the internal state.
    pub fn update_u64(&mut self, val: u64) {
        self.state = self.combine(self.state, seahash::hash(&val.to_le_bytes()));
    }

    /// Hashes a byte slice and updates the internal state.
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.state = self.combine(self.state, seahash::hash(bytes));
    }

    /// Returns the final hash value.
    pub fn finish(&self) -> u64 {
        self.state
    }

    /// Combines two hashes in a way that respects order and avoids XOR cancellations.
    fn combine(&self, a: u64, b: u64) -> u64 {
        // A simple but effective combination function
        // Similar to boost::hash_combine but using 64-bit constants
        a.wrapping_add(b)
            .wrapping_add(0x9E3779B97F4A7C15)
            .rotate_left(7)
    }
}

/// A standard way to combine two hashes without creating a full Hasher if not needed.
pub fn combine_hashes(a: u64, b: u64) -> u64 {
    let mut h = Hasher { state: a };
    h.update_u64(b);
    h.finish()
}

/// Helper for hashing primitives.
pub fn hash_u64(val: u64) -> u64 {
    seahash::hash(&val.to_le_bytes())
}

pub fn hash_f32(val: f32) -> u64 {
    hash_u64(val.to_bits() as u64)
}

pub fn hash_str(val: &str) -> u64 {
    seahash::hash(val.as_bytes())
}
