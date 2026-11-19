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
        "budget" => commands::budget_set(&collect(args)),
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
    println!("  carbotrack export --format json|csv|report-json [--output FILE] 导出记录");
    println!("  carbotrack budget set --daily GRAMS                 设置日碳预算");
    println!("  carbotrack dashboard [--port PORT]                  启动本地面板");
    println!("  carbotrack serve [--port PORT] [--host HOST]        启动静态面板服务");
}
