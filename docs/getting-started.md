# Getting Started

## Install

```bash
git clone https://github.com/totalwindupflightsystems/warpfs.git
cd warpfs
cargo build --release
./target/release/warpfs-cli --help
```

### Requirements

- Rust 1.80+
- `libfuse3-dev` (for FUSE mount)
- `attr` package (for `getfattr` / `setfattr`)

```bash
# Ubuntu/Debian
sudo apt install libfuse3-dev attr

# macOS (FUSE not supported; CLI + MCP still work)
# No additional deps needed for CLI-only use
```

## First Run

```bash
# 1. Initialize WarpFS in your project
cd my-project
warpfs init

# 2. Build the dependency graph
warpfs graph discover

# 3. Auto-classify every file
warpfs classify

# 4. Explore
warpfs graph stats
warpfs graph impact sys:some-header.h --max-depth 3
warpfs graph related src/main.rs --relation imports
```

## Using with AI Agents

### Via MCP (Claude Desktop, Hermes, Continue)

```bash
warpfs serve --mcp
```

Add to your MCP client configuration:

```json
{
  "mcpServers": {
    "warpfs": {
      "command": "/path/to/warpfs-cli",
      "args": ["serve", "--mcp"],
      "cwd": "/path/to/your/project"
    }
  }
}
```

### Via FUSE Mount

```bash
mkdir /mnt/vfs
warpfs mount /mnt/vfs

# Standard tools work through the mount
ls /mnt/vfs/
cat /mnt/vfs/src/main.rs
getfattr -n user.vfs.role /mnt/vfs/src/main.rs
```
