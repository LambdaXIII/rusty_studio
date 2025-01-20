# Change Log

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- **Project Structure:**
  - 添加了 `CHANGELOG.md` 文件，用于记录项目变更。
  
- **Core Module:**
  - 在 `DataBox` 中添加了详细的使用示例和注释，帮助用户更好地理解和使用该结构。
  - 在 `Time` 结构中增加了更多方法的详细注释和使用示例，包括 `to_second`、`milliseconds_from_seconds`、`from_seconds` 等方法。
  - 在 `Timebase` 结构中增加了更多方法的详细注释和使用示例，包括 `from_real_fps`、`milliseconds_from_frames` 和 `frames_from_milliseconds` 方法。
  - 在 `TimecodeParts` 结构中增加了更多方法的详细注释和使用示例，包括 `from_timecode`、`from_timestamp`、`to_timecode` 和 `to_timestamp` 方法。

- **Library Module:**
  - 在 `lib.rs` 中为 `core` 和 `timeline` 模块添加了详细的模块描述。

### Changed

- **Project Configuration:**
  - 修改了 `Cargo.toml` 文件中的库名称，从 `rstu` 改为 `rusty_studio`。
  
- **Core Module:**
  - 修改了 `core.rs` 文件中对 `timecode_parts` 模块的引用方式，从 `pub mod timecode_parts;` 改为 `mod timecode_parts;` 并在文件末尾显式导出 `timecode_parts` 模块。
  - 修改了 `time.rs` 文件中对 `Time` 结构的注释，使其更加清晰明确，并修正了一些拼写错误。
  - 修改了 `timebase.rs` 文件中对 `Timebase` 结构的注释，使其更加清晰明确，并修正了一些拼写错误。
  - 修改了 `timecode_parts.rs` 文件中对 `TimecodeParts` 结构的注释，使其更加清晰明确，并修正了一些拼写错误。

- **Tests:**
  - 删除了 `tests/test_time.rs` 文件，因为其测试用例已经整合到各个模块的单元测试中。

### Removed

- **Tests:**
  - 删除了 `tests/test_time.rs` 文件，因为其测试用例已经整合到各个模块的单元测试中。
