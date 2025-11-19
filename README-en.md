# EsolangIDE

A lightweight Esolang IDE built with **Rust**.
It supports interpreter execution, debugging, breakpoint control, and provides unified interfaces for interpreters and debug sessions to support multiple esoteric programming languages.

## Tech Stack

* **UI Framework**: egui / eframe
* **Language**: Rust 2024

## Quick Start

```bash
# Clone the project
git clone https://github.com/iAeternus/esolang-ide.git
cd esolang-ide

# Build and run
cargo run --release
```

## Development Plan

### Core System

* [ ] UI / Core module separation
* [ ] Interpreter & debugger interfaces
* [ ] Single-step execution (Step)
* [ ] Breakpoint support (instruction index)
* [ ] Multi-interpreter registration & management
* [ ] External interpreter adapter (process-based)
* [ ] Configuration & project management (TOML)

### UI Display & Interaction

* [ ] Editor / Output panel / Debug panel
* [ ] Line number display
* [ ] Breakpoint gutter click support
* [ ] State visualization panel (memory/tape)

### Extension

* [ ] Plugin-based interpreter extension mechanism

## License

This project is licensed under the [MIT License](LICENSE).

**Author**: [@iAeternus](https://github.com/iAeternus)
