# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 25.0.0-dev.5 - 2025-04-25

## 25.0.0-dev.4 - 2025-04-25

## 25.0.0-dev.3 - 2025-04-21

## 25.0.0-dev.2 - 2025-04-21

## 25.0.0-dev.1 - 2025-04-14

## 25.0.0-dev.0 - 2025-01-01

## 24.0.0-dev.22 - 2024-10-17

## 24.0.0-dev.21 - 2024-10-09
### Changed
- Flow: Upgrade to [Protoflow 0.4.1]

## 24.0.0-dev.20 - 2024-10-01
### Changed
- Flow: Upgrade to [Protoflow 0.4.0]

## 24.0.0-dev.19 - 2024-09-25
### Changed
- Flow: Upgrade to [Protoflow 0.3.1]

## 24.0.0-dev.18 - 2024-09-20

## 24.0.0-dev.17 - 2024-09-19

## 24.0.0-dev.16 - 2024-09-17

## 24.0.0-dev.15 - 2024-09-12

## 24.0.0-dev.14 - 2024-09-09
### Changed
- Flow: Upgrade to [Protoflow 0.3.0]

## 24.0.0-dev.13 - 2024-09-07

## 24.0.0-dev.12 - 2024-09-06

## 24.0.0-dev.11 - 2024-09-06

## 24.0.0-dev.10 - 2024-09-05

## 24.0.0-dev.9 - 2024-09-05
### Changed
- Flow: Upgrade to [Protoflow 0.2.1]

## 24.0.0-dev.8 - 2024-08-21
### Changed
- Flow: Upgrade to [Protoflow 0.2.0]

## 24.0.0-dev.7 - 2024-08-09
### Added
- Flow: `MpscTransport` transport implementation
- Flow: `#[derive(Block)]` now implements `sysml_model` traits

## 24.0.0-dev.6 - 2024-08-02
### Changed
- Flow: Blocks now return a `BlockResult` (a typedef for `Result<(), BlockError>`)
- Flow: `InputPort#receive()` now renamed to `InputPort#recv()`
### Added
- Flow: `BlockError::Panic` error
- Flow: `Buffer` block type
- Flow: `Message` trait, combining `prost::Message`, `Clone`, and `Default`
- Flow: `System::build()` DSL
- Flow: `Transport` interface, and `MockTransport` implementation

## 24.0.0-dev.5 - 2024-07-29

## 24.0.0-dev.4 - 2024-07-26
### Changed
- Flow: `Block#execute` parameters have changed

## 24.0.0-dev.3 - 2024-07-25
### Fixed
- Flow: fix build issues due to the `alloc` crate
### Added
- Flow: mock transport for testing
- Flow: `Const` and `Random` blocks

## 24.0.0-dev.2 - 2024-07-24
### Added
- Flow: `derive` feature
- Flow: `#[derive(Block)]` macro
- Flow: `#[derive(FunctionBlock)]` macro
- Flow: `BlockDescriptor` trait
- Flow: `BlockError` enum
- Flow: `PortError` enum
### Changed
- Flow: scheduler methods now return a `Result`

## 24.0.0-dev.1 - 2024-07-23
### Changed
- Flow: `Block#execute` now returns a `Result`
- Flow: `Block#execute` now takes a parameter

## 24.0.0-dev.0 - 2024-06-29

[Protoflow 0.4.1]: https://github.com/asimov-platform/protoflow/compare/0.4.0...0.4.1
[Protoflow 0.4.0]: https://github.com/asimov-platform/protoflow/compare/0.3.1...0.4.0
[Protoflow 0.3.1]: https://github.com/asimov-platform/protoflow/compare/0.3.0...0.3.1
[Protoflow 0.3.0]: https://github.com/asimov-platform/protoflow/compare/0.2.1...0.3.0
[Protoflow 0.2.1]: https://github.com/asimov-platform/protoflow/compare/0.2.0...0.2.1
[Protoflow 0.2.0]: https://github.com/asimov-platform/protoflow/compare/0.1.0...0.2.0
