# CarboTrack

给你的代码装上碳表 — 一行代码追踪应用碳足迹。

CarboTrack 是嵌入式碳排放追踪 SDK，基于 Rust 核心，覆盖 CPU、GPU、内存、磁盘、网络等维度。

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

## 路线图

- M1: Rust 核心模型、追踪器、本地存储、碳强度数据库
- M2: 多语言绑定、框架集成、CLI、碳预算告警
- M3: 可视化面板、热点分析、合规报告
- M4: 文档、多平台构建与发布

# CarboTrack

M1 implementation PR.
