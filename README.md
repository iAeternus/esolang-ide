# EsolangIDE

一个使用 **Rust** 构建的轻量级 Esolang IDE。
支持解释器运行、调试、断点控制等功能，提供解释器和 debug 会话统一接口以支持多种 esolang。

## 技术选型

* **UI 框架**：egui / eframe
* **语言**：Rust 2024

## 快速开始

```bash
# 克隆项目
git clone https://github.com/iAeternus/esolang-ide.git
cd esolang-ide

# 构建并运行
cargo run --release
```

## 开发计划

### 核心系统

* [ ] UI / Core 模块分离
* [ ] 解释器与调试器接口
* [ ] 单步执行（Step）
* [ ] 断点支持（指令索引）
* [ ] 多解释器注册与管理
* [ ] 外部解释器适配（进程调用）
* [ ] 配置与项目管理（TOML）

### UI 展示与交互

* [ ] 编辑器 / 输出区域 / 调试面板
* [ ] 行号显示
* [ ] 断点 gutter 点击触发
* [ ] 状态可视化面板（memory/tape）

### 扩展

* [ ] 插件式解释器扩展机制

## 许可证

此项目根据 [MIT 许可证](LICENSE) 授权。

**作者**: [@iAeternus](https://github.com/iAeternus)
