//! `warpfs graph discover` and `warpfs graph stats`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use warpfs_graph::{GraphDB, Parser};
use warpfs_metadata::inventory::{self, Edge};

/// Directory names to skip when walking for Go source files.
const SKIP_DIRS: &[&str] = &[
    "target", // Rust build output
    "node_modules",
    "vendor",
];

/// Walk the current directory for `*.go` files, parse their imports, and write
/// the resulting edges to both `.vfs/graph/edges.jsonl` and the DuckDB graph
/// database at `.vfs/graph/graph.db`.
pub fn run_discover() -> Result<()> {
    let cwd =
        std::env::current_dir().context("failed to determine the current directory")?;

    // Collect every .go file under the current directory.
    let mut go_files = Vec::new();
    collect_go_files(&cwd, &mut go_files)
        .context("failed to walk directory tree for Go files")?;

    // Parse imports from each file.
    let mut parser =
        Parser::new().context("failed to initialize the tree-sitter Go parser")?;
    let mut all_edges: Vec<Edge> = Vec::new();
    let mut unique_sources: HashSet<String> = HashSet::new();

    for file in &go_files {
        let source = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(_) => continue, // skip unreadable files
        };
        let rel = file
            .strip_prefix(&cwd)
            .unwrap_or(file)
            .to_string_lossy()
            .into_owned();

        let edges = parser
            .parse_imports(&rel, &source)
            .with_context(|| format!("failed to parse {rel}"))?;

        for e in &edges {
            unique_sources.insert(e.from.clone());
        }
        all_edges.extend(edges);
    }

    // Persist edges to the JSONL inventory file.
    let edges_jsonl = cwd.join(".vfs").join("graph").join("edges.jsonl");
    inventory::append_edges(&edges_jsonl, &all_edges)
        .context("failed to write edges.jsonl")?;

    // Populate the DuckDB graph database.
    let graph_db = cwd.join(".vfs").join("graph").join("graph.db");
    let graph_db_str = graph_db.to_str().unwrap_or(".vfs/graph/graph.db");
    let graph = GraphDB::open(graph_db_str).context("failed to open DuckDB graph database")?;
    graph
        .insert_edges(&all_edges)
        .context("failed to insert edges into DuckDB")?;

    let n = all_edges.len();
    let m = unique_sources.len();
    println!("Discovered {n} edges across {m} files");
    Ok(())
}

/// Print summary statistics from the DuckDB graph database.
///
/// If no graph data exists yet, a helpful message is printed instead.
pub fn run_stats() -> Result<()> {
    let cwd =
        std::env::current_dir().context("failed to determine the current directory")?;
    let graph_db = cwd.join(".vfs").join("graph").join("graph.db");

    // Avoid touching DuckDB when there is nothing to read.
    if !graph_db.exists() {
        println!("No graph data. Run `warpfs graph discover` first.");
        return Ok(());
    }

    let graph_db_str = graph_db.to_str().unwrap_or(".vfs/graph/graph.db");
    let graph = GraphDB::open(graph_db_str).context("failed to open DuckDB graph database")?;
    let stats = graph.stats().context("failed to compute graph statistics")?;

    if stats.total_edges == 0 {
        println!("No graph data. Run `warpfs graph discover` first.");
        return Ok(());
    }

    println!("Total edges: {}", stats.total_edges);
    println!("Unique source files: {}", stats.unique_files);
    println!("Unique dependencies: {}", stats.unique_dependencies);
    println!("Top dependencies:");
    for (dep, count) in &stats.top_dependencies {
        println!("  {dep}: {count}");
    }

    Ok(())
}

/// Recursively collect `*.go` file paths under `dir`, skipping hidden entries
/// and the directories listed in [`SKIP_DIRS`].
fn collect_go_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip hidden files/directories (e.g. .vfs, .git) and known build dirs.
        if name_str.starts_with('.') || SKIP_DIRS.contains(&name_str.as_ref()) {
            continue;
        }

        let ft = entry.file_type()?;
        if ft.is_dir() {
            collect_go_files(&path, out)?;
        } else if ft.is_file() && path.extension().map(|e| e == "go").unwrap_or(false) {
            out.push(path);
        }
    }
    Ok(())
}
