use crate::commands::{budget_show, demo_tracker, export, report};
use carbotrack_core::{NetworkScope, ReportFormat};

pub fn run() -> Result<(), String> {
    let tracker = demo_tracker();

    tracker.cpu("api_request", 150.0, 2);
    tracker.gpu("inference", 800.0, 1);
    tracker.memory("cache", 4.0, 3_600_000.0);
    tracker.disk("snapshot", 2.0, carbotrack_core::DiskMedia::Ssd);
    tracker.network("upload", 0.05, NetworkScope::Local);
    tracker.network("sync", 1.2, NetworkScope::CrossRegion);

    println!(
        "演示数据已生成，区域: us-east-1，共 {} 条记录",
        tracker.records().len()
    );
    budget_show(&tracker);
    report::print(&tracker.report(ReportFormat::GhgProtocol, 0, u128::MAX));
    export::write_json(&tracker, None)?;

    let hotspots = carbotrack_core::analysis::analyze(&tracker.records());
    let suggestions = carbotrack_core::analysis::suggest(&hotspots);
    println!("减排建议:");
    for (index, suggestion) in suggestions.iter().enumerate() {
        println!("  {}. {}", index + 1, suggestion.message);
    }
    Ok(())
}
