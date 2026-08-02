# AGENTS.md

This file provides guidance to AI coding agents (Claude Code, etc.) when working with code in this repository. CLAUDE.md is a symlink to this file.

This repository is the AGPL-3.0 licensed Ultralytics template for new Rust projects. It is published to crates.io as `ultralytics-template-rust` and ships a minimal library plus binary crate, example tests and benchmarks, and the standard Ultralytics CI, formatting, and publishing workflows to copy and adapt.

## Core Principles (CRITICAL)

**Less is more. The simplest solution is the best solution.** The action hierarchy for every change: **Delete > Replace > Add**.

1. **Solve at the owner**: Put behavior in the code path that owns or observes it. For fixes, never guard a symptom with a staleness check, initialization flag, skip-first-call branch, or `try/except` around broken logic; relocate the trigger and delete the wrong path. For features, extend the existing owner rather than creating a parallel abstraction.
2. **Search and reuse first**: Search the whole repository before creating a feature, component, helper, workflow, or utility. Reuse or adapt what exists, consolidate in-scope duplication in the shared owner, and delete duplicate paths. Three similar lines beat a helper nobody else calls.
3. **Delete and modify existing code before creating new code**: Bugfixes are net-negative by default unless deletion and relocation are demonstrably impossible. A new file must first prove it cannot fit cleanly in an existing owner.
4. **Keep scope minimal**: Implement only the simplest complete solution. Avoid impossible-state handling, speculative flags, compatibility shims, policy scaffolding, and unrelated cleanup. Tests are out of scope by default — rely on existing coverage and focused validation; only an uncovered, high-risk regression path justifies minimal new test code.
5. **Ship zero-regression, production-ready changes**: Understand what you remove instead of retaining broken code as insurance. Remove unused imports, functions, types, files, and comments; run relevant cleanup checks; and thoroughly debug and validate the changed owner. Do not break existing features or workflows unless the PR intentionally removes them with evidence.

**Review gate:** for every addition, the reviewer decides whether deleting or changing existing code would have fixed the problem instead — if it would, that is a blocking finding. A missing or thin PR description is never itself a finding.

NEVER push to `main`. NEVER force push. Always start work in a new git worktree (`git worktree add`) on a feature branch and open a PR — never edit the primary checkout directly, it may hold in-flight work.

## PR Workflow

After opening a PR:

1. Wait for the automated PR review and auto-format commit from Ultralytics Actions (`format.yml`), then pull and address every finding.
2. Review the full diff in-session against the Core Principles, performance, and the review gate above, then batch the fixes into one commit and push. After each round of bot or human commits, pull and resume the same reviewer on `<last-reviewed-sha>..HEAD` plus anything that delta could have invalidated. Repeat until the local head matches the live head.
3. Hand off or merge only on a clean final pass: one cold full-diff review returning LGTM with no findings, on a head that is still live at merge time.
4. Never fight other commits: Ultralytics Actions pushes auto-format and header commits, and multiple users may work on the same PR. `git pull --rebase` before pushing; never reset or revert commits you did not author.
5. After the PR merges, clean up: remove local worktrees and branches for it, then `git checkout main && git pull`.

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
