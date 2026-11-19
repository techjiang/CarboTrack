mod cpu;
mod disk;
mod gpu;
mod memory;
mod network;

use crate::carbon::{
    calculate, CarbonIntensity, EmissionBreakdown, EmissionRecord, Factors, Operation, Report,
    ReportFormat,
};
use crate::storage::MemoryStore;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarboTrackConfig {
    pub cloud: String,
    pub region: String,
    pub budget_gco2e_per_day: Option<f64>,
    pub factors: Factors,
}

impl Default for CarboTrackConfig {
    fn default() -> Self {
        Self {
            cloud: String::new(),
            region: "global".into(),
            budget_gco2e_per_day: None,
            factors: Factors::default(),
        }
    }
}

#[derive(Clone)]
pub struct CarboTrack {
    config: CarboTrackConfig,
    intensity: CarbonIntensity,
    records: Arc<Mutex<MemoryStore>>,
}

impl Default for CarboTrack {
    fn default() -> Self {
        Self::new(CarboTrackConfig::default())
    }
}

impl CarboTrack {
    pub fn new(config: CarboTrackConfig) -> Self {
        let intensity = CarbonIntensity::from_region(&config.region);
        Self {
            config,
            intensity,
            records: Arc::new(Mutex::new(MemoryStore::new())),
        }
    }

    pub fn config(&self) -> &CarboTrackConfig {
        &self.config
    }
    pub fn intensity(&self) -> CarbonIntensity {
        self.intensity
    }

    pub fn track(&self, operation: Operation) -> EmissionRecord {
        let breakdown = calculate(&operation, self.intensity, self.config.factors);
        let record = EmissionRecord {
            id: format!(
                "ct-{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            ),
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            operation,
            breakdown,
        };
        self.records
            .lock()
            .expect("storage lock poisoned")
            .push(record.clone());
        record
    }

    pub fn records(&self) -> Vec<EmissionRecord> {
        self.records.lock().expect("storage lock poisoned").all()
    }

    pub fn emissions_between(&self, from_ms: u128, to_ms: u128) -> f64 {
        self.records
            .lock()
            .expect("storage lock poisoned")
            .between(from_ms, to_ms)
            .iter()
            .map(|r| r.breakdown.total())
            .sum()
    }

    pub fn report(&self, format: ReportFormat, from_ms: u128, to_ms: u128) -> Report {
        let records = self
            .records
            .lock()
            .expect("storage lock poisoned")
            .between(from_ms, to_ms);
        let total: f64 = records.iter().map(|r| r.breakdown.total()).sum();
        let title = match format {
            ReportFormat::GhgProtocol => "GHG Protocol Emission Report",
            ReportFormat::Iso14064 => "ISO 14064 Emission Report",
        };
        Report {
            format,
            total_gco2e: total,
            record_count: records.len(),
            body: format!(
                "{title}\nTotal: {total:.4} gCO2e\nRecords: {}\n",
                records.len()
            ),
        }
    }

    pub fn total_breakdown(&self) -> EmissionBreakdown {
        self.records
            .lock()
            .expect("storage lock poisoned")
            .all()
            .iter()
            .fold(EmissionBreakdown::zero(), |mut acc, r| {
                acc.add_assign(r.breakdown);
                acc
            })
    }

    pub fn budget_exceeded(&self) -> bool {
        match self.config.budget_gco2e_per_day {
            Some(budget) => self.total_breakdown().total() > budget,
            None => false,
        }
    }

    pub fn cpu(
        &self,
        operation: impl Into<String>,
        duration_ms: f64,
        cores: u32,
    ) -> EmissionRecord {
        self.track(cpu::operation(operation, duration_ms, cores))
    }

    pub fn gpu(
        &self,
        operation: impl Into<String>,
        duration_ms: f64,
        cards: u32,
    ) -> EmissionRecord {
        self.track(gpu::operation(operation, duration_ms, cards))
    }

    pub fn memory(
        &self,
        operation: impl Into<String>,
        gb: f64,
        duration_ms: f64,
    ) -> EmissionRecord {
        self.track(memory::operation(operation, gb, duration_ms))
    }

    pub fn disk(
        &self,
        operation: impl Into<String>,
        gb: f64,
        media: crate::carbon::DiskMedia,
    ) -> EmissionRecord {
        self.track(disk::operation(operation, gb, media))
    }

    pub fn network(
        &self,
        operation: impl Into<String>,
        gb: f64,
        scope: crate::carbon::NetworkScope,
    ) -> EmissionRecord {
        self.track(network::operation(operation, gb, scope))
    }
}
