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

pub fn summary(suggestions: &[Suggestion]) -> String {
    if suggestions.is_empty() {
        return "当前碳排较低，暂无高优减排项".into();
    }
    suggestions
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{}. {}", index + 1, item.message))
        .collect::<Vec<_>>()
        .join("\n")
}
