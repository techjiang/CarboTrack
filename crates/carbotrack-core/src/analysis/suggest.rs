use crate::analysis::Hotspot;

#[derive(Debug, Clone, PartialEq)]
pub struct Suggestion {
    pub message: String,
}

pub fn suggest(hotspots: &[Hotspot]) -> Vec<Suggestion> {
    hotspots
        .iter()
        .take(3)
        .map(|h| Suggestion {
            message: format!("优化高频操作「{}」以降低碳排放", h.operation),
        })
        .collect()
}
