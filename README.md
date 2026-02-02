# todo-app

Rust 编写的跨平台待办事项工具，从 CLI 起步，逐步演进为桌面应用。

## 状态

- **当前**：功能完整的 CLI 工具（`cargo install` 可用）
- **目标**：统一数据模型 + CLI/GUI 双前端（Tauri）

## 特性

- 任务增删改查
- 本地 JSON 持久化（遵循 XDG 标准）
- 跨平台（Windows / macOS / Linux）
- 零外部依赖（仅 Rust std + serde + clap）

## Roadmap

### 短期（v0.2–v0.3）
- [ ] 支持任务优先级（`high/medium/low`）
- [ ] 添加 `--due <date>` 截止日期字段
- [ ] 实现 `list --pending` / `--completed` 过滤
- [ ] 单元测试覆盖核心逻辑（≥80%）

### 中期（v0.4–v1.0）
- [ ] 初始化 Tauri 项目（`src-tauri/`）
- [ ] 共享 Rust domain logic（`lib.rs` 抽离 CLI 与 GUI 公共模块）
- [ ] 基础桌面 UI（任务列表 + 新增表单）
- [ ] 自动同步 CLI 与 GUI 数据存储路径

### 长期（v1.0+）
- [ ] 托盘集成（macOS Menu Bar / Windows System Tray）
- [ ] 本地通知（任务到期提醒）
- [ ] 导出为 Markdown / CSV
- [ ] 插件系统（自定义命令、Webhook）

## 构建

```bash
# CLI
cargo build --release

# （未来）Desktop
cd src-tauri && cargo tauri build
