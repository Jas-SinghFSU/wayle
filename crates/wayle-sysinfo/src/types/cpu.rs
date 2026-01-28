use derive_more::Debug;

/// CPU metrics snapshot.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CpuData {
    /// Total CPU usage across all cores (0.0 - 100.0).
    pub usage_percent: f32,

    /// Average frequency across all cores in MHz.
    pub avg_frequency_mhz: u64,

    /// Maximum frequency among all cores in MHz.
    pub max_frequency_mhz: u64,

    /// Frequency of the busiest core (highest usage) in MHz.
    pub busiest_core_freq_mhz: u64,

    /// CPU temperature in Celsius, if available.
    pub temperature_celsius: Option<f32>,

    /// Per-core CPU data.
    #[debug(skip)]
    pub cores: Vec<CoreData>,
}

/// Per-core CPU metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct CoreData {
    /// Core identifier (e.g., "cpu0", "cpu1").
    pub name: String,

    /// Core usage percentage (0.0 - 100.0).
    pub usage_percent: f32,

    /// Core frequency in MHz.
    pub frequency_mhz: u64,
}
