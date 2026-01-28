/// Memory metrics snapshot.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MemoryData {
    /// Total physical memory in bytes.
    pub total_bytes: u64,

    /// Used memory in bytes.
    pub used_bytes: u64,

    /// Available memory in bytes.
    pub available_bytes: u64,

    /// Memory usage percentage (0.0 - 100.0).
    pub usage_percent: f32,

    /// Total swap space in bytes.
    pub swap_total_bytes: u64,

    /// Used swap in bytes.
    pub swap_used_bytes: u64,

    /// Swap usage percentage (0.0 - 100.0).
    pub swap_percent: f32,
}
