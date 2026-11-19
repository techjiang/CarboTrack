use crate::carbon::{NetworkScope, Operation};

pub fn operation(operation: impl Into<String>, gb: f64, scope: NetworkScope) -> Operation {
    Operation::Network {
        operation: operation.into(),
        gb,
        scope,
    }
}
