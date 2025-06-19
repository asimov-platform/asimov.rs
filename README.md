# ASIMOV Software Development Kit (SDK) for Rust

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-module)](https://crates.io/crates/asimov-module)
[![Documentation](https://docs.rs/asimov-module/badge.svg)](https://docs.rs/asimov-module/)

[ASIMOV] is a polyglot development platform for trustworthy, neurosymbolic AI.

🚧 _We are building in public. This is presently under heavy construction._

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add asimov-module
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
asimov = { package = "asimov-module", "version" = "25.0.0-dev.11" }
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
asimov = { package = "asimov-module", "version" = "25.0.0-dev.11", default-features = false, features = ["tracing"] }
```

## 👉 Examples

### Importing the SDK

```rust
use asimov::*;
```

## 📚 Reference

[docs.rs/asimov-module/](https://docs.rs/asimov-module/)

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-platform/asimov.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-platform/asimov.rs&text=ASIMOV%20Software%20Development%20Kit%20%28SDK%29%20for%20Rust)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-platform/asimov.rs&title=ASIMOV%20Software%20Development%20Kit%20%28SDK%29%20for%20Rust)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-platform/asimov.rs&t=ASIMOV%20Software%20Development%20Kit%20%28SDK%29%20for%20Rust)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-platform/asimov.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-platform/asimov.rs)

[ASIMOV]: https://asimov.sh
