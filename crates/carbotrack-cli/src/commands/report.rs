use carbotrack_core::{EmissionBreakdown, Report, ReportFormat};

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

pub fn print_breakdown(breakdown: &EmissionBreakdown) {
    println!("碳排构成:");
    for (name, value) in breakdown.by_category() {
        println!("  {name}: {value:.6} gCO2e");
    }
}

pub fn as_markdown(report: &Report) -> String {
    format!(
        "# {}\n\n- 总碳排: {:.4} gCO2e\n- 记录数: {}\n\n{}",
        report_title(report),
        report.total_gco2e,
        report.record_count,
        report.body.trim_end()
    )
}

pub fn report_title(report: &Report) -> &'static str {
    match report.format {
        ReportFormat::GhgProtocol => "GHG Protocol Emission Report",
        ReportFormat::Iso14064 => "ISO 14064 Emission Report",
    }
}

pub fn as_json(report: &Report) -> String {
    format!(
        "{{\"format\":\"{:?}\",\"total_gco2e\":{:.6},\"record_count\":{},\"body\":{}}}",
        report.format,
        report.total_gco2e,
        report.record_count,
        json_escape(&report.body),
    )
}

fn json_escape(input: &str) -> String {
    let mut output = String::from("\"");
    for ch in input.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            c => output.push(c),
        }
    }
    output.push('"');
    output
}

pub fn value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == key)
        .and_then(|index| args.get(index + 1))
        .cloned()
}
