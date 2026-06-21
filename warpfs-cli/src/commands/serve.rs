//! `warpfs serve --mcp` — MCP server stub.
//!
//! The real MCP server lives in the `warpfs-mcp` crate (not yet implemented).

use anyhow::Result;

/// Print the not-yet-implemented stub message.
pub fn run(_mcp: bool) -> Result<()> {
    println!("MCP server not yet implemented. Coming in warpfs-mcp crate.");
    Ok(())
}
