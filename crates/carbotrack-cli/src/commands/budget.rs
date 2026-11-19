use crate::commands::report::{print_breakdown, value};
use carbotrack_core::CarboTrack;

pub fn set(args: &[String]) -> Result<(), String> {
    let daily = value(args, "--daily").ok_or("缺少 --daily 参数")?;
    let budget: f64 = daily.parse().map_err(|_| "碳预算必须是数字")?;
    println!("日碳预算已设置为 {budget:.2} gCO2e（演示配置）");
    Ok(())
}

pub fn show(tracker: &CarboTrack) {
    let breakdown = tracker.total_breakdown();
    let budget = tracker.config().budget_gco2e_per_day.unwrap_or(0.0);
    println!("当前碳排: {:.6} gCO2e", breakdown.total());
    print_breakdown(&breakdown);
    println!("日碳预算: {:.2} gCO2e", budget);
    if tracker.budget_exceeded() {
        println!("预算状态: 已超标");
    } else {
        println!("预算状态: 正常");
    }
}
