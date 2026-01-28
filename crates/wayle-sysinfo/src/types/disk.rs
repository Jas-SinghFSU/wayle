use std::path::PathBuf;

/// Disk metrics for a single mount point.
#[derive(Debug, Clone, PartialEq)]
pub struct DiskData {
    /// Mount point path (e.g., "/", "/home").
    pub mount_point: PathBuf,

    /// Filesystem name (e.g., "ext4", "btrfs").
    pub filesystem: String,

    /// Total disk space in bytes.
    pub total_bytes: u64,

    /// Used disk space in bytes.
    pub used_bytes: u64,

    /// Available disk space in bytes.
    pub available_bytes: u64,

    /// Disk usage percentage (0.0 - 100.0).
    pub usage_percent: f32,
}
