use crate::carbon::Operation;

pub fn operation(operation: impl Into<String>, duration_ms: f64, cards: u32) -> Operation {
    Operation::Gpu {
        operation: operation.into(),
        duration_ms,
        cards,
    }
}
