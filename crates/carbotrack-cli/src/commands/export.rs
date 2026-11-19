use crate::commands::report::value;
use carbotrack_core::CarboTrack;
use std::fs;
use std::path::Path;

pub fn run(args: &[String]) -> Result<(), String> {
    let tracker = crate::commands::demo_tracker();
    let format = value(args, "--format").unwrap_or_else(|| "json".into());
    let output = value(args, "--output");
    match format.as_str() {
        "json" => write_json(&tracker, output.as_deref()),
        "csv" => write_csv(&tracker, output.as_deref()),
        "report-json" => write_report_json(&tracker, output.as_deref()),
        "report-md" => write_report_markdown(&tracker, output.as_deref()),
        other => Err(format!("不支持的导出格式: {other}")),
    }
}

pub fn write_report_json(tracker: &CarboTrack, output: Option<&str>) -> Result<(), String> {
    let report = tracker.report(carbotrack_core::ReportFormat::GhgProtocol, 0, u128::MAX);
    write(output, crate::commands::report::as_json(&report))
}

pub fn write_report_markdown(tracker: &CarboTrack, output: Option<&str>) -> Result<(), String> {
    let report = tracker.report(carbotrack_core::ReportFormat::GhgProtocol, 0, u128::MAX);
    write(output, crate::commands::report::as_markdown(&report))
}

fn write(output: Option<&str>, content: String) -> Result<(), String> {
    match output {
        Some(path) => fs::write(path, content).map_err(|e| format!("写入 {path} 失败: {e}")),
        None => {
            print!("{content}");
            Ok(())
        }
    }
}

pub fn write_json(tracker: &CarboTrack, output: Option<&str>) -> Result<(), String> {
    let records = tracker.records();
    let mut json = String::from("[\n");
    for (index, record) in records.iter().enumerate() {
        if index > 0 {
            json.push_str(",\n");
        }
        json.push_str(&format!(
            "  {{\"id\":\"{}\",\"timestamp_ms\":{},\"total_gco2e\":{:.6}}}",
            record.id,
            record.timestamp_ms,
            record.breakdown.total()
        ));
    }
    json.push_str("\n]\n");
    write(output, json)
}

pub fn write_csv(tracker: &CarboTrack, output: Option<&str>) -> Result<(), String> {
    if let Some(path) = output {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {e}"))?;
            }
        }
    }
    let mut csv = String::from("id,timestamp_ms,operation,total_gco2e\n");
    for record in tracker.records() {
        csv.push_str(&format!(
            "{},{},{:?},{:.6}\n",
            record.id,
            record.timestamp_ms,
            record.operation,
            record.breakdown.total()
        ));
    }
    write(output, csv)
}
