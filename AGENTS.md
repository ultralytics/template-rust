# AGENTS.md

This file provides guidance to AI coding agents (Claude Code, etc.) when working with code in this repository. CLAUDE.md is a symlink to this file.

## Core Principles (CRITICAL)

Respecting these principles is critical for every PR.

**Less is more. The simplest solution is the best solution.**

The action hierarchy for every change: **Delete > Replace > Add**. The best code change is a deletion. The second best is modifying what exists. Adding new code is the last resort.

1. **Minimal**: The simplest solution that works. Do not over-engineer, over-abstract, or add code just in case. Three similar lines beat a premature abstraction. Avoid error handling for impossible states, feature flags, compatibility shims, or policy scaffolding unless they are truly required.
2. **Solve at the source**: Do not hack fixes. Solve problems at their root. If something is broken, fix or remove the broken thing. Never patch over a broken abstraction, add workarounds, or add synchronization code for state that should not be duplicated.
3. **Delete ruthlessly**: When replacing code, delete what it replaced. Remove unused imports, functions, types, files, and commented-out code. Git preserves history. Run the repo's relevant dead-code or cleanup check when available.
4. **Replace > Add**: Modify existing code over adding new code. Edit existing files, extend existing components or functions with minimal parameters, and reuse existing utilities. If creating a new file, first prove it cannot fit cleanly in an existing file.
5. **Check existing**: Search the entire repo before creating anything new. If a feature, component, helper, responder, workflow, or utility already solves a similar problem, reuse or adapt it and delete the duplicate path.
6. **Deduplicate**: Do not duplicate existing code when updating the repo. Consolidate or refactor duplicates you find when it is in scope and low risk.
7. **Zero Regression**: Do not break existing features or workflows unless the PR intentionally removes them with evidence.
8. **Production ready**: All changes must be thoroughly debugged, validated, and production ready.

**When fixing bugs, ask: "What can I delete?" before "What can I replace?" before "What should I add?"**

## PR Workflow

After opening a PR:

1. Wait for the automated PR review and auto-format commit from Ultralytics Actions (`format.yml`), then pull and address every finding.
2. Launch an independent adversarial review agent with cold context (just the PR diff and this file) to hunt for bugs, regressions, and Core Principles violations. Fix, push, and repeat with a fresh agent until one reports LGTM.
3. Never fight other commits: Ultralytics Actions pushes auto-format and header commits, and multiple users may work on the same PR. `git pull --rebase` before pushing; never force-push, reset, or revert commits you did not author.
4. After the PR merges, clean up: remove local worktrees and branches for it, then `git checkout main && git pull`.

## Commands

```bash
cargo test --all --all-features                                          # run all tests (unit, integration, doc tests)
cargo test example_output_matches_cli                                    # run one test by name
cargo fmt --all                                                          # format (nightly rustfmt via rust-toolchain.toml)
cargo clippy --all-targets --all-features -- -D warnings                 # lint (CI fails on any warning)
cargo bench                                                              # criterion benchmarks (benches/example_bench.rs)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info # coverage (CI command)
cargo deny check                                                         # license/advisory/source checks (deny.toml)
```

Toolchain: `rust-toolchain.toml` pins nightly locally because `rustfmt.toml` uses unstable options, but CI runs clippy and tests on stable and `Cargo.toml` sets `rust-version = "1.91"` — keep code stable-compatible. CI matrix is ubuntu/macos/windows.

## Architecture

This is the Ultralytics template for new Rust projects — a minimal lib + bin crate meant to be copied and adapted. `src/lib.rs` exposes the example API (`add_numbers()`, `run_example()`) with doc-tested examples and unit tests; `src/main.rs` is a thin CLI that prints `run_example()`, and integration tests (`tests/basic.rs`) validate both library behavior and the spawned CLI's output. `benches/example_bench.rs` is a criterion benchmark (`harness = false` in `Cargo.toml`). `format.yml` runs Ultralytics Actions on PRs (Prettier, codespell, link checks, AI labels/summaries, plus a nightly `cargo fmt` check) and commits fixes back to the PR branch.

Publishing: `publish.yml` (push to main or manual dispatch, upstream repo only) publishes to crates.io when the `Cargo.toml` version has no existing `v<version>` git tag.

## Conventions

- License headers (`// Ultralytics 🚀 AGPL-3.0 License - https://ultralytics.com/license` in Rust files) are added automatically by Ultralytics Actions — don't add or revert them manually.
- Import style is enforced by `rustfmt.toml`: `StdExternalCrate` grouping with module granularity; comments wrap at 120.
- Edition 2024. Public APIs get doc comments, with runnable examples where useful (doc tests run in CI).
