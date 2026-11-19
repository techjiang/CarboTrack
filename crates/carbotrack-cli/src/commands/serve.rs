use crate::commands::report::value;
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(args: &[String]) -> Result<(), String> {
    let port = value(args, "--port").unwrap_or_else(|| "8080".into());
    let port: u16 = port.parse().map_err(|_| "端口必须是数字")?;
    let host = value(args, "--host").unwrap_or_else(|| "127.0.0.1".into());
    serve_only(&host, port)
}

pub fn serve_only(host: &str, port: u16) -> Result<(), String> {
    let dist = dashboard_dir()?;
    let listener =
        TcpListener::bind((host, port)).map_err(|e| format!("端口 {port} 启动失败: {e}"))?;
    let url = format!("http://{host}:{port}");
    println!("CarboTrack dashboard: {url}");

    if open_browser(&url) {
        println!("已尝试打开浏览器");
    }

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                use std::io::{Read, Write};
                let mut buffer = [0; 2048];
                let _ = stream.read(&mut buffer);
                let path = String::from_utf8_lossy(&buffer)
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("/")
                    .to_string();
                let relative = path.trim_start_matches('/');
                let file = if relative.is_empty() {
                    dist.join("index.html")
                } else {
                    dist.join(relative)
                };
                let (status, content_type, body) = serve_file(&file, &dist);
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(&body);
            }
            Err(error) => eprintln!("连接错误: {error}"),
        }
    }
    Ok(())
}

fn serve_file(path: &Path, dist: &Path) -> (&'static str, &'static str, Vec<u8>) {
    let safe = path.starts_with(dist);
    if !safe || !path.is_file() {
        return (
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"Not Found".to_vec(),
        );
    }
    let content_type = match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        _ => "application/octet-stream",
    };
    match fs::read(path) {
        Ok(body) => ("200 OK", content_type, body),
        Err(_) => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"Not Found".to_vec(),
        ),
    }
}

fn dashboard_dir() -> Result<PathBuf, String> {
    let current = std::env::current_dir().map_err(|e| e.to_string())?;
    let candidates = [
        current.join("dashboard/dist"),
        current.join("../../dashboard/dist"),
    ];
    candidates
        .into_iter()
        .find(|path| path.join("index.html").is_file())
        .ok_or_else(|| "找不到 dashboard/dist，请先构建面板".into())
}

fn open_browser(url: &str) -> bool {
    for (program, args) in [
        ("xdg-open", vec![url]),
        ("chromium", vec![url]),
        ("google-chrome", vec![url]),
    ] {
        if Command::new(program)
            .args(args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .is_ok()
        {
            return true;
        }
    }
    false
}
