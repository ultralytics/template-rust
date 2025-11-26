<a href="https://www.ultralytics.com/"><img src="https://raw.githubusercontent.com/ultralytics/assets/main/logo/Ultralytics_Logotype_Original.svg" width="320" alt="Ultralytics logo"></a>

# Documentation Directory (`docs/`)

This directory contains documentation for the [Ultralytics](https://github.com/ultralytics/ultralytics) Rust template. It
is intentionally lightweight so teams can layer on their preferred tooling without clutter.

## 📖 Overview

- **rustdoc-first:** Inline `///` comments in `src/` power API docs (`cargo doc --open`).
- **Markdown pages:** Expand project guides here using Markdown for quick start notes, architecture decisions, and API
  walkthroughs.
- **Optional generators:** Teams can add [mdBook](https://rust-lang.github.io/mdBook/) or `cargo doc` deploy workflows as
  needed; keep configs in the repo root to avoid nesting.

## 🚀 Getting Started

To work with the documentation locally:

1. **Browse API docs:** `cargo doc --open` renders the crate documentation locally.
2. **Preview Markdown:** Use your preferred Markdown preview (VS Code, mdBook, or static site generator) to iterate on
   guides stored here.
3. **Ship docs:** If you add mdBook or other tooling, document the commands in this file so contributors know how to
   build and deploy the site.

Update this directory as the Rust code evolves to keep examples aligned and onboarding smooth.

## 🙌 Contributing

Contributions to improve the documentation are welcome! Whether it's fixing typos, clarifying explanations, adding
examples, or translating content, your help is valuable. Please see our
[Contributing Guide](https://docs.ultralytics.com/help/contributing/) for more details on how to get started. You can also
ask questions on the [Ultralytics Community Forums](https://community.ultralytics.com/).
