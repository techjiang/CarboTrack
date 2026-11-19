use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Factors {
    pub cpu_kwh_per_core_minute: f64,
    pub gpu_kwh_per_card_minute: f64,
    pub memory_kwh_per_gb_hour: f64,
    pub ssd_kwh_per_gb: f64,
    pub hdd_kwh_per_gb: f64,
    pub network_local_kwh_per_gb: f64,
    pub network_cross_region_kwh_per_gb: f64,
    pub cdn_kwh_per_gb: f64,
    pub pue: f64,
}

impl Default for Factors {
    fn default() -> Self {
        Self {
            cpu_kwh_per_core_minute: 0.0004,
            gpu_kwh_per_card_minute: 0.0025,
            memory_kwh_per_gb_hour: 0.00037,
            ssd_kwh_per_gb: 0.0004,
            hdd_kwh_per_gb: 0.00065,
            network_local_kwh_per_gb: 0.0002,
            network_cross_region_kwh_per_gb: 0.006,
            cdn_kwh_per_gb: 0.0005,
            pue: 1.0,
        }
    }
}
