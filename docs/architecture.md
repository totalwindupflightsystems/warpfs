# Architecture

## Overview

```
┌──────────────────────────────────────────┐
│  Agent (Claude / Hermes / Codex)         │
│    │  MCP tools  │  getfattr  │  cat     │
├────┼─────────────┼────────────┼──────────┤
│  WarpFS                                  │
│  ┌──────────┐  ┌──────────┐  ┌────────┐ │
│  │  FUSE    │  │   MCP    │  │  CLI   │ │
│  │  mount   │  │  server  │  │  shim  │ │
│  └────┬─────┘  └────┬─────┘  └───┬────┘ │
│       │              │            │      │
│  ┌────┴──────────────┴────────────┴────┐ │
│  │          Metadata Engine            │ │
│  │  xattrs │ JSONL edges │ DuckDB      │ │
│  └────────────────┬────────────────────┘ │
│                   │                      │
│  ┌────────────────┴────────────────────┐ │
│  │      Backend Storage                │ │
│  │  Git repos │ S3 │ local disk        │ │
│  └─────────────────────────────────────┘ │
└──────────────────────────────────────────┘
```

## Three Layers

### 1. Interface Layer

How agents connect to WarpFS:

| Interface | Protocol | Use Case |
|-----------|----------|----------|
| **FUSE mount** | Kernel VFS | `ls`, `cat`, `getfattr` through standard tools |
| **MCP server** | JSON-RPC over stdio | Direct agent integration (Claude, Hermes) |
| **CLI shim** | Subprocess | Scripts, CI, one-off queries |

### 2. Metadata Engine

Metadata lives *outside* file content:

- **xattrs** (`user.vfs.*`) — per-file metadata: role, status, backend, hash
- **JSONL inventory** (`.vfs/graph/edges.jsonl`) — dependency graph edges, append-only, git-friendly
- **DuckDB** (`.vfs/graph/graph.db`) — queryable graph, rebuilt from JSONL on mount
- **Manifest** (`.vfs/manifest.yaml`) — master config: backends, permissions, rules, sandbox

**Content is never modified.** The agent reads file bytes exactly as they
exist on disk. WarpFS adds metadata alongside, not inside.

### 3. Backend Storage

Virtual folders map to real storage:

| Backend | Example | Features |
|---------|---------|----------|
| Local disk | `src/` | Direct passthrough, writable |
| Git remote | `github.com/org/repo` | Auto-pull, read-only or writable |
| S3 bucket | `my-bucket/prefix/` | Read-only, write-through with auto-upload |

## Key Design Decisions

### Metadata, not injection

v1 injected context into file comments. The model treated it as actual
file bytes. v2 stores everything in xattrs and inventory files.

### JSONL as source of truth

Edges are append-only JSONL. Git-friendly (diffs are readable), streamable,
splittable. DuckDB is a rebuildable query cache — the JSONL is authoritative.

### MCP as universal fallback

When agent tools don't expose xattrs (browser-based agents, remote agents),
the MCP server provides `vfs_get_metadata`, `vfs_graph_related`, etc.

## Data Flow

```
File write detected (inotify)
  → AST re-parse (tree-sitter)
  → Edge extraction (imports, calls, tests)
  → Deduplication
  → Append to edges.jsonl
  → INSERT OR IGNORE into DuckDB
  → Set xattrs (user.vfs.role, user.vfs.status)
  → Fire triggers (WASM plugins)
```
