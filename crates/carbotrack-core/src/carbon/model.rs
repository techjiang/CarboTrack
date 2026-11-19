use crate::carbon::{CarbonIntensity, Factors};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Operation {
    Compute {
        operation: String,
        duration_ms: f64,
        cores: u32,
    },
    Gpu {
        operation: String,
        duration_ms: f64,
        cards: u32,
    },
    Memory {
        operation: String,
        gb: f64,
        duration_ms: f64,
    },
    Disk {
        operation: String,
        gb: f64,
        media: DiskMedia,
    },
    Network {
        operation: String,
        gb: f64,
        scope: NetworkScope,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiskMedia {
    Ssd,
    Hdd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkScope {
    Local,
    CrossRegion,
    Cdn,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EmissionBreakdown {
    pub compute: f64,
    pub storage: f64,
    pub network: f64,
    pub cooling: f64,
}

impl EmissionBreakdown {
    pub fn total(&self) -> f64 {
        self.compute + self.storage + self.network + self.cooling
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmissionRecord {
    pub id: String,
    pub timestamp_ms: u128,
    pub operation: Operation,
    pub breakdown: EmissionBreakdown,
}

pub fn calculate(
    operation: &Operation,
    intensity: CarbonIntensity,
    factors: Factors,
) -> EmissionBreakdown {
    let mut compute = 0.0;
    let mut storage = 0.0;
    let mut network = 0.0;

    match operation {
        Operation::Compute {
            duration_ms, cores, ..
        } => {
            let minutes = duration_ms / 60_000.0;
            compute = f64::from(*cores)
                * minutes
                * factors.cpu_kwh_per_core_minute
                * intensity.gco2e_per_kwh;
        }
        Operation::Gpu {
            duration_ms, cards, ..
        } => {
            let minutes = duration_ms / 60_000.0;
            compute = f64::from(*cards)
                * minutes
                * factors.gpu_kwh_per_card_minute
                * intensity.gco2e_per_kwh;
        }
        Operation::Memory {
            gb, duration_ms, ..
        } => {
            let hours = duration_ms / 3_600_000.0;
            compute = *gb * hours * factors.memory_kwh_per_gb_hour * intensity.gco2e_per_kwh;
        }
        Operation::Disk { gb, media, .. } => {
            let kwh_per_gb = match media {
                DiskMedia::Ssd => factors.ssd_kwh_per_gb,
                DiskMedia::Hdd => factors.hdd_kwh_per_gb,
            };
            storage = *gb * kwh_per_gb * intensity.gco2e_per_kwh;
        }
        Operation::Network { gb, scope, .. } => {
            let kwh_per_gb = match scope {
                NetworkScope::Local => factors.network_local_kwh_per_gb,
                NetworkScope::CrossRegion => factors.network_cross_region_kwh_per_gb,
                NetworkScope::Cdn => factors.cdn_kwh_per_gb,
            };
            network = *gb * kwh_per_gb * intensity.gco2e_per_kwh;
        }
    }

    let cooling = (compute + storage) * (factors.pue - 1.0);
    EmissionBreakdown {
        compute,
        storage,
        network,
        cooling,
    }
}
