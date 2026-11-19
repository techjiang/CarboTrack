# CarboTrack v1.0.1 Release

给你的代码装上碳表 — 一行代码追踪应用碳足迹。

## 本次发布内容

- Rust 核心：CPU / GPU / 内存 / 磁盘 / 网络碳排计算模型
- CLI：demo / report / export / budget / dashboard / serve
- 面板：总碳排、趋势、构成、预算、热点分析与减排建议
- 导出：JSON、CSV、报告 JSON、Markdown
- 文档：上线可用性说明与项目用途说明
- 截图：`screenshots/carbotrack-dashboard-final-release.png`

## 用途

把应用运行时的资源消耗换算成碳排放，帮助团队定位高碳操作、设置碳预算并导出报告，用于工程优化与内部 ESG 统计。

## 说明

该版本适合本地演示和内网试用。生产用途仍需补齐 SQLite 持久化、多语言原生绑定包和多平台 CI 制品。
