use crate::carbon::Operation;

pub fn operation(operation: impl Into<String>, gb: f64, duration_ms: f64) -> Operation {
    Operation::Memory {
        operation: operation.into(),
        gb,
        duration_ms,
    }
}
