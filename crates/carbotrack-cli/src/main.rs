mod commands;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return ExitCode::SUCCESS;
    };

    let result = match command.as_str() {
        "demo" => commands::demo(),
        "report" => commands::report(&collect(args)),
        "export" => commands::export(&collect(args)),
        "budget" => {
            let args = collect(args);
            match args.first().map(String::as_str) {
                Some("set") => commands::budget_set(&args[1..]),
                Some("show") => {
                    commands::budget_show(&crate::commands::demo_tracker());
                    Ok(())
                }
                Some(other) => Err(format!("未知的 budget 子命令: {other}")),
                None => Err("缺少 budget 子命令，可用 set 或 show".into()),
            }
        }
        "dashboard" => commands::dashboard(&collect(args)),
        "serve" => commands::serve(&collect(args)),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        other => Err(format!("未知命令: {other}")),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("错误: {message}");
            ExitCode::FAILURE
        }
    }
}

fn collect(args: impl Iterator<Item = String>) -> Vec<String> {
    args.collect()
}

fn print_help() {
    println!("CarboTrack CLI 1.0.0");
    println!();
    println!("用法:");
    println!("  carbotrack demo                                     生成演示数据");
    println!("  carbotrack report [--format ghg-protocol|iso-14064] 生成碳排报告");
    println!("  carbotrack export --format json|csv|report-json|report-md [--output FILE] 导出记录");
    println!("  carbotrack budget set --daily GRAMS                 设置日碳预算");
    println!("  carbotrack budget show                              查看演示预算状态");
    println!("  carbotrack dashboard [--port PORT]                  启动本地面板");
    println!("  carbotrack serve [--port PORT] [--host HOST]        启动静态面板服务");
}
