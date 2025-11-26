<a href="https://www.ultralytics.com/"><img src="https://raw.githubusercontent.com/ultralytics/assets/main/logo/Ultralytics_Logotype_Original.svg" width="320" alt="Ultralytics logo"></a>

# 🛠 Ultralytics Rust Project Template

Welcome to the Ultralytics Rust Project Template! This repository provides a standardized foundation for initiating Rust
projects at [Ultralytics](https://www.ultralytics.com/). It incorporates best practices in project structure,
configuration, and essential tooling to streamline development. By using this template, Ultralytics developers can
ensure consistency, maintain high quality standards, and accelerate the setup process for new Rust-based software.

[![Template CI](https://github.com/ultralytics/template/actions/workflows/ci.yml/badge.svg)](https://github.com/ultralytics/template/actions/workflows/ci.yml)
[![Ultralytics Actions](https://github.com/ultralytics/template/actions/workflows/format.yml/badge.svg)](https://github.com/ultralytics/template/actions/workflows/format.yml)
[![Publish](https://github.com/ultralytics/template/actions/workflows/publish.yml/badge.svg)](https://github.com/ultralytics/template/actions/workflows/publish.yml)
[![codecov](https://codecov.io/gh/ultralytics/template/graph/badge.svg?token=K9IunpFzjS)](https://codecov.io/gh/ultralytics/template)

[![Ultralytics Discord](https://img.shields.io/discord/1089800235347353640?logo=discord&logoColor=white&label=Discord&color=blue)](https://discord.com/invite/ultralytics)
[![Ultralytics Forums](https://img.shields.io/discourse/users?server=https%3A%2F%2Fcommunity.ultralytics.com&logo=discourse&label=Forums&color=blue)](https://community.ultralytics.com/)
[![Ultralytics Reddit](https://img.shields.io/reddit/subreddit-subscribers/ultralytics?style=flat&logo=reddit&logoColor=white&label=Reddit&color=blue)](https://reddit.com/r/ultralytics)

## 🗂️ Repository Structure

This template is organized for intuitive navigation and a clear understanding of Rust project components.

- `src/lib.rs`: Core library code for the crate.
- `src/main.rs`: Example binary entrypoint that exercises the library.
- `tests/`: Integration tests executed with `cargo test`.
- `docs/`: Optional Markdown docs that complement Rustdoc output.
- `Cargo.toml`: Package metadata, dependencies, and workspace configuration.
- `.gitignore`: Git ignore rules tailored for Cargo builds and IDEs.
- `LICENSE`: Project license file (default AGPL-3.0-or-later).
- `.github/workflows/`: GitHub Actions for CI, formatting, and publishing the crate.

```plaintext
your-project/
│
├── src/                       # Library and binaries
│   ├── lib.rs
│   └── main.rs
├── tests/                     # Integration tests
│   └── basic.rs
├── docs/                      # Additional Markdown docs (optional)
│   └── README.md
├── .github/workflows/         # CI, formatting, publish pipelines
│   ├── ci.yml
│   ├── format.yml
│   └── publish.yml
├── .gitignore                 # Git ignore rules for Rust
├── Cargo.toml                 # Cargo package metadata
├── LICENSE                    # Project license file
└── README.md                  # This file
```

### 📦 Source Code Directory (`src/`)

The `src/` directory contains both the reusable library (`lib.rs`) and an example binary (`main.rs`). Expand the library
with modules as the crate grows, and keep binaries thin wrappers that delegate to library functions.

### 🧪 Testing Directory (`tests/`)

Integration tests in `tests/` validate user-facing behavior. Add unit tests alongside the code they cover in `src/` and
run everything with `cargo test`.

### 📚 Documentation Directory (`docs/`)

Keep Markdown guides in `docs/` for onboarding and architecture notes. Rustdoc generated from `src/` remains the source
of truth for API documentation (`cargo doc --open`).

## ✨ Starting a New Project

Kickstart your new Rust project using this template:

1. **Create Your Repository:** Click "Use this template" on GitHub to generate a new repository. Update the remote URL in
   your local clone.
2. **Customize:** Adjust `Cargo.toml` (package name, description, authors), refresh this `README.md`, and tailor
   `.github/workflows/*.yml` as needed.
3. **Develop:** Add modules to `src/lib.rs`, binaries to `src/bin/` or `src/main.rs`, and integration tests to `tests/`.
4. **Validate:** Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` locally.
5. **Document:** Keep code-level docs up to date with `///` comments and expand `docs/` for guides or ADRs.

## 🚀 Quickstart

Install Rust via [rustup](https://rustup.rs/) and then:

```bash
cargo fmt
cargo test
cargo run
```

Using the published crate from another project (after release):

```toml
# Cargo.toml
[dependencies]
ultralytics-template-rust = "0.1.0"
```

```rust
use ultralytics_template_rust::add_numbers;

fn main() {
    let sum = add_numbers(1, 2);
    println!("Sum: {sum}");
}
```

## 🔧 Utilizing the Template

For Ultralytics team members and external contributors:

- Clone the newly created repository based on this template to start working locally.
- Keep `README.md` and `Cargo.toml` aligned with the crate’s purpose and public API.
- Extend CI jobs if you add features like WebAssembly targets, feature-flag matrices, or documentation publishing.

With this template, Ultralytics aims to foster a culture of excellence and uniformity in Rust software development,
ensuring every project starts on a solid foundation aligned with industry standards and organizational best practices.

## 💡 Contribute

Ultralytics thrives on community collaboration, and we deeply value your contributions! Whether it's reporting bugs,
suggesting features, or submitting code changes, your involvement is crucial.

- **Reporting Issues:** Encounter a bug? Please report it on [GitHub Issues](https://github.com/ultralytics/template/issues).
- **Feature Requests:** Have an idea for improvement? Share it via [GitHub Issues](https://github.com/ultralytics/template/issues).
- **Pull Requests:** Want to contribute code? Please read our [Contributing Guide](https://docs.ultralytics.com/help/contributing/) first, then submit a Pull Request.
- **Feedback:** Share your thoughts and experiences by participating in our official [Survey](https://www.ultralytics.com/survey?utm_source=github&utm_medium=social&utm_campaign=Survey).

A heartfelt thank you 🙏 goes out to all our contributors! Your efforts help make Ultralytics tools better for everyone.

[![Ultralytics open-source contributors](https://raw.githubusercontent.com/ultralytics/assets/main/im/image-contributors.png)](https://github.com/ultralytics/ultralytics/graphs/contributors)

## 📄 License

Ultralytics offers two licensing options to accommodate diverse needs:

- **AGPL-3.0 License**: Ideal for students, researchers, and enthusiasts passionate about open collaboration and
  knowledge sharing. This [OSI-approved](https://opensource.org/license/agpl-v3) open-source license promotes
  transparency and community involvement. See the [LICENSE](LICENSE) file for details.
- **Enterprise License**: Designed for commercial applications, this license permits the seamless integration of
  Ultralytics software and AI models into commercial products and services, bypassing the copyleft requirements of
  AGPL-3.0. For commercial use cases, please inquire about an
  [Ultralytics Enterprise License](https://www.ultralytics.com/license).

## 📮 Contact

For bug reports or feature suggestions related to this template or other Ultralytics projects, please use
[GitHub Issues](https://github.com/ultralytics/template/issues). For general questions, discussions, and community
support, join our [Discord](https://discord.com/invite/ultralytics) server!

<br>
<div align="center">
  <a href="https://github.com/ultralytics"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-github.png" width="3%" alt="Ultralytics GitHub"></a>
  <img src="https://github.com/ultralytics/assets/raw/main/social/logo-transparent.png" width="3%" alt="space">
  <a href="https://www.linkedin.com/company/ultralytics/"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-linkedin.png" width="3%" alt="Ultralytics LinkedIn"></a>
  <img src="https://github.com/ultralytics/assets/raw/main/social/logo-transparent.png" width="3%" alt="space">
  <a href="https://twitter.com/ultralytics"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-twitter.png" width="3%" alt="Ultralytics Twitter"></a>
  <img src="https://github.com/ultralytics/assets/raw/main/social/logo-transparent.png" width="3%" alt="space">
  <a href="https://youtube.com/ultralytics?sub_confirmation=1"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-youtube.png" width="3%" alt="Ultralytics YouTube"></a>
  <img src="https://github.com/ultralytics/assets/raw/main/social/logo-transparent.png" width="3%" alt="space">
  <a href="https://www.tiktok.com/@ultralytics"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-tiktok.png" width="3%" alt="Ultralytics TikTok"></a>
  <img src="https://github.com/ultralytics/assets/raw/main/social/logo-transparent.png" width="3%" alt="space">
  <a href="https://ultralytics.com/bilibili"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-bilibili.png" width="3%" alt="Ultralytics BiliBili"></a>
  <img src="https://github.com/ultralytics/assets/raw/main/social/logo-transparent.png" width="3%" alt="space">
  <a href="https://discord.com/invite/ultralytics"><img src="https://github.com/ultralytics/assets/raw/main/social/logo-social-discord.png" width="3%" alt="Ultralytics Discord"></a>
</div>
