# CarboTrack

给你的代码装上碳表 — 一行代码追踪应用碳足迹。

CarboTrack 是嵌入式碳排放追踪 SDK，基于 Rust 核心，覆盖 CPU、GPU、内存、磁盘、网络等维度。本地优先，不上传第三方。

一句话说用处：把应用运行时的资源消耗换算成碳排放，帮助开发团队看到“哪段代码更费碳”、设置碳预算，并导出报告用于内部 ESG 或工程优化。它不是碳交易平台，也不是云厂商账单系统，而是开发者可嵌入的碳足迹计量工具。

## 上线可用性说明

当前版本已具备可演示、可本地运行的上线基础：

- Rust workspace 可构建 core 与 CLI，核心模型含单元测试。
- CLI 提供 `demo`、`report`、`export`、`budget`、`dashboard`、`serve` 命令。
- 面板为纯静态资源，无 CDN 依赖，可由 `carbotrack serve` 本地托管。
- 报告支持 GHG Protocol / ISO 14064 摘要，数据导出支持 JSON、CSV、报告 JSON、Markdown。
- 数据默认留在本地，适合离线演示与内网环境。

生产使用前仍需补齐 SQLite 持久化、多语言原生绑定发布包和 CI 多平台制品；路线图见文末 M3 / M4。

## 快速开始

```rust
use carbotrack_core::{CarboTrack, CarboTrackConfig, NetworkScope};

let tracker = CarboTrack::new(CarboTrackConfig {
    region: "us-east-1".into(),
    ..Default::default()
});
tracker.cpu("api_request", 150.0, 2);
tracker.network("upload", 0.05, NetworkScope::Local);
println!("{:?}", tracker.total_breakdown());
```

## CLI

```bash
cargo run -p carbotrack-cli -- demo
cargo run -p carbotrack-cli -- report --format ghg-protocol
cargo run -p carbotrack-cli -- export --format csv --output emissions.csv
cargo run -p carbotrack-cli -- export --format report-json --output report.json
cargo run -p carbotrack-cli -- export --format report-md --output report.md
cargo run -p carbotrack-cli -- budget set --daily 500
cargo run -p carbotrack-cli -- budget show
cargo run -p carbotrack-cli -- dashboard --port 8080
```

`dashboard` 会启动一个本地静态面板，展示总碳排、趋势、构成、预算、热点与冷却占比；冷却排放通过 `EmissionBreakdown::cooling_emissions()` 读取，冷却占比按“冷却排放 / 总排放”计算，生产构建位于 `dashboard/dist`。

本地校验面板产物一致性：

```bash
node dashboard/scripts/verify.mjs
```

## 目录结构

- `crates/carbotrack-core`：碳排计算模型、追踪器、存储与分析
- `crates/carbotrack-cli`：命令行工具、报告、导出、本地面板
- `dashboard`：React 风格静态可视化面板（无外部 CDN 依赖）
- `examples`：Node.js Express / Python FastAPI 接入示例

## 路线图

- M1 ✅ Rust 核心模型、追踪器、本地存储、碳强度数据库
- M2 ✅ CLI、JSON/CSV/报告导出、预算状态、冷却占比、本地可视化面板、框架示例
- M3 多语言原生绑定、合规报告文件导出、SQLite 持久化
- M4 文档、多平台构建与发布
