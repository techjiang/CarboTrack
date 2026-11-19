use crate::carbon::Operation;

pub fn operation(operation: impl Into<String>, duration_ms: f64, cores: u32) -> Operation {
    Operation::Compute {
        operation: operation.into(),
        duration_ms,
        cores,
    }
}
