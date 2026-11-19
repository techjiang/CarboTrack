use crate::carbon::EmissionRecord;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Hotspot {
    pub operation: String,
    pub total_gco2e: f64,
}

pub fn analyze(records: &[EmissionRecord]) -> Vec<Hotspot> {
    let mut totals: BTreeMap<String, f64> = BTreeMap::new();
    for record in records {
        let key = match &record.operation {
            crate::carbon::Operation::Compute { operation, .. }
            | crate::carbon::Operation::Gpu { operation, .. }
            | crate::carbon::Operation::Memory { operation, .. }
            | crate::carbon::Operation::Disk { operation, .. }
            | crate::carbon::Operation::Network { operation, .. } => operation.clone(),
        };
        *totals.entry(key).or_default() += record.breakdown.total();
    }
    let mut result: Vec<Hotspot> = totals
        .into_iter()
        .map(|(operation, total_gco2e)| Hotspot {
            operation,
            total_gco2e,
        })
        .collect();
    result.sort_by(|a, b| {
        b.total_gco2e
            .partial_cmp(&a.total_gco2e)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    result
}
