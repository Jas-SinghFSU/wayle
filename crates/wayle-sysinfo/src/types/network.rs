/// Network interface metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkData {
    /// Interface name (e.g., "eth0", "wlan0").
    pub interface: String,

    /// Bytes received since last poll.
    pub rx_bytes: u64,

    /// Bytes transmitted since last poll.
    pub tx_bytes: u64,

    /// Receive rate in bytes per second.
    pub rx_bytes_per_sec: u64,

    /// Transmit rate in bytes per second.
    pub tx_bytes_per_sec: u64,
}
