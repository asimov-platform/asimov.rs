# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
