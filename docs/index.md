# WarpFS Documentation

WarpFS is an agent-first virtual filesystem. It pre-computes a dependency
graph across your codebase and exposes it through standard filesystem
tools and an MCP server — so AI agents can answer structural questions
without reading files.

## Guides

- **[Getting Started](getting-started.md)** — install, init, first discover
- **[Architecture](architecture.md)** — FUSE, metadata engine, backends
- **[CLI Reference](cli-reference.md)** — every command and flag
- **[MCP Tools](mcp-tools.md)** — agent integration via JSON-RPC
- **[Graph Engine](graph-engine.md)** — AST parsing, DuckDB, impact analysis

## Quick Links

- [GitHub Repository](https://github.com/totalwindupflightsystems/warpfs)
- [Design Document](https://totalwindupflightsystems.github.io/reports/hermes-vfs-design.html)
- [Value Test Report](https://totalwindupflightsystems.github.io/reports/warpfs-value-test.html)

## Supported Languages

Go, Python, TypeScript, Rust, JavaScript, Java, C, C++, Ruby
