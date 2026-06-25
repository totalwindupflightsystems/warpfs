# Contributing to WarpFS

## Development Setup

```bash
git clone https://github.com/totalwindupflightsystems/warpfs.git
cd warpfs

# Build
cargo build

# Run tests
cargo test --workspace

# Format + lint
cargo fmt --check
cargo clippy --workspace -- -D warnings
```

Requirements: Rust 1.80+, `libfuse3-dev`, `attr`.

## Project Structure

```
warpfs/
├── warpfs-core/        # Manifest, workspace, sandbox, virtual dirs
├── warpfs-metadata/    # xattr read/write, JSONL inventory
├── warpfs-graph/       # Tree-sitter parsers, DuckDB queries, classify
├── warpfs-mcp/         # JSON-RPC MCP server (8 tools)
├── warpfs-cli/         # CLI (init, meta, graph, classify, mount, serve)
├── warpfs-fuse/        # FUSE daemon, ops, permissions
├── warpfs-backends/    # S3, Git, local storage backends
├── warpfs-triggers/    # inotify watchers
├── warpfs-plugins/     # Extism WASM plugin runtime
├── warpfs-permissions/ # Mode-bit enforcement engine
└── specs/              # Design documents
```

## Commit Convention

```
feat(<crate>): <brief description>

Co-authored-by: wojons <wojonstech@gmail.com>
```

Crate name matches Cargo.toml `name` field: `warpfs_core`, `warpfs_graph`,
`warpfs_metadata`, `warpfs_cli`, `warpfs_mcp`, `warpfs_backends`,
`warpfs_triggers`, `warpfs_fuse`, `warpfs_plugins`, `warpfs_permissions`.

## Pre-commit Checks

WarpFS uses [GitReins](https://github.com/totalwindupflightsystems/gitreins)
for pre-commit enforcement:

- **Tier 1** (blocks commit): secrets scan, `cargo test`, LSP diagnostics,
  static analysis
- **Tier 2** (post-commit): LLM-based semantic evaluation

All commits must pass Tier 1 guards. Install the hook:

```bash
pip install gitreins
gitreins init
```

## Pull Requests

1. Run `cargo fmt` and `cargo clippy` before pushing
2. Add tests for new functionality
3. Update CHANGELOG.md under `[Unreleased]`
4. PR title follows commit convention above

## Adding a Language

1. Add the `tree-sitter-<lang>` crate to `warpfs-graph/Cargo.toml`
2. Add the variant to `Language` enum in `warpfs-graph/src/parser.rs`
3. Add extension → language mapping in `from_extension()`
4. Implement import extraction in `parse_imports()`
5. Add entrypoint + test patterns to `warpfs-graph/src/classify.rs`
6. Add test files for verification
