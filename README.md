# ASIMOV Software Development Kit (SDK)

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-sdk)](https://crates.io/crates/asimov-sdk)
[![Package on NPM](https://img.shields.io/npm/v/asimov.js)](https://npmjs.com/package/asimov.js)
[![Package on Pub.dev](https://img.shields.io/pub/v/asimov)](https://pub.dev/packages/asimov)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov.py)](https://pypi.org/project/asimov.py)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov.rb)](https://rubygems.org/gems/asimov.rb)

**[ASIMOV] is a polyglot development platform for [trustworthy neurosymbolic machine intelligence].**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

<br/>

## ✨ Features

- Available both as the command-line tool [`asimov`] and a polyglot library.
- Defines [flow-based] [program patterns] for refining data into knowledge.
- Implements a [module system] enabling an ecosystem of [modules].
- Enables dataflow systems through reusable components called blocks.
- Polyglot software <sup><sub>(soon!)</sub></sup> available for Dart, Python, Ruby, Rust, and TypeScript.
- Cuts red tape: 100% free and unencumbered public domain software.

## ⬇️ Installation

<details>
<summary>Installation for Rust from Crates.io</summary>

#### Installation from [Crates.io]

```bash
cargo add asimov-sdk --rename asimov
```
</details>

<details>
<summary>Installation for JavaScript/TypeScript from NPM</summary>

#### Installation from [NPM]

```bash
npm install asimov.js@dev
bun add asimov.js
pnpm add asimov.js
yarn add asimov.js
```
</details>

<details>
<summary>Installation for Dart from Pub.dev</summary>

#### Installation from [Pub.dev]

```bash
dart pub add asimov
flutter pub add asimov
```
</details>

<details>
<summary>Installation for Python from PyPI</summary>

#### Installation from [PyPI]

```bash
pip install -U asimov.py
uv add asimov.py
poetry add asimov.py
pdm add asimov.py
```
</details>

<details>
<summary>Installation for Ruby from RubyGems</summary>

#### Installation from [RubyGems]

```bash
gem install asimov.rb
bundle add asimov.rb
```
</details>

## 👉 Examples

## 📚 Reference

### Glossary

- **Module**: A collection of systems and blocks, packaged as a reusable unit.

- **System**: A collection of blocks that are connected together.
  Systems are the top-level entities in dataflow programs.

- **Block**: An encapsulated system component that processes messages.
  Blocks are the autonomous units of computation in a system.

- **Port**: A named connection point on a block that sends or receives
  messages. Ports are the only interfaces through which blocks communicate
  with each other.

- **Message**: A unit of data that flows between blocks in a system, from port
  to port.

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-platform/asimov-sdk.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https%3A%2F%2Fgithub.com%2Fasimov-platform%2Fasimov-sdk&text=ASIMOV%20Software%20Development%20Kit%20%28SDK%29)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Fasimov-platform%2Fasimov-sdk&title=ASIMOV%20Software%20Development%20Kit%20%28SDK%29)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https%3A%2F%2Fgithub.com%2Fasimov-platform%2Fasimov-sdk&t=ASIMOV%20Software%20Development%20Kit%20%28SDK%29)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fgithub.com%2Fasimov-platform%2Fasimov-sdk)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Fasimov-platform%2Fasimov-sdk)

[`asimov`]: https://github.com/asimov-platform/asimov-cli

[Crates.io]: https://crates.io/crates/asimov-sdk
[NPM]: https://npmjs.com/package/asimov.js
[Pub.dev]: https://pub.dev/packages/asimov
[PyPI]: https://pypi.org/project/asimov.py
[RubyGems]: https://rubygems.org/gems/asimov.rb

[ASIMOV]: https://asimov.sh
[Cargo]: https://rustup.rs
[flow-based]: https://github.com/artob/awesome-fbp
[modules]: https://github.com/asimov-modules
[module system]: https://asimov-specs.github.io
[program patterns]: https://asimov-specs.github.io/program-patterns/
[trustworthy neurosymbolic machine intelligence]: https://asimov.blog/introducing-asimov/
