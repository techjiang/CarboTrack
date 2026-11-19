use crate::carbon::EmissionRecord;

#[derive(Debug, Default)]
pub struct MemoryStore {
    records: Vec<EmissionRecord>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, record: EmissionRecord) {
        self.records.push(record);
    }
    pub fn all(&self) -> Vec<EmissionRecord> {
        self.records.clone()
    }
    pub fn between(&self, from_ms: u128, to_ms: u128) -> Vec<EmissionRecord> {
        self.records
            .iter()
            .filter(|r| r.timestamp_ms >= from_ms && r.timestamp_ms <= to_ms)
            .cloned()
            .collect()
    }
}
