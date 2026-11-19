use carbotrack_core::{Report, ReportFormat};

pub fn run(args: &[String]) -> Result<(), String> {
    let tracker = crate::commands::demo_tracker();
    let format = value(args, "--format").unwrap_or_else(|| "ghg-protocol".into());
    let format = match format.as_str() {
        "ghg-protocol" => ReportFormat::GhgProtocol,
        "iso-14064" => ReportFormat::Iso14064,
        other => return Err(format!("不支持的报告格式: {other}")),
    };
    print(&tracker.report(format, 0, u128::MAX));
    Ok(())
}

pub fn print(report: &Report) {
    print!("{}", report.body);
}

pub fn value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == key)
        .and_then(|index| args.get(index + 1))
        .cloned()
}
