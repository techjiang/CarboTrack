use crate::carbon::{DiskMedia, Operation};

pub fn operation(operation: impl Into<String>, gb: f64, media: DiskMedia) -> Operation {
    Operation::Disk {
        operation: operation.into(),
        gb,
        media,
    }
}
