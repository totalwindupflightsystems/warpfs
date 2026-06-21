//! Transitive impact analysis — find all files that depend on a given file, directly or transitively.

use std::collections::{HashSet, VecDeque};

use duckdb::{params, Connection};
use serde::Serialize;

use crate::error::GraphResult;

/// A single file in the impact chain.
#[derive(Debug, Clone, Serialize)]
pub struct ImpactFile {
    /// The file path.
    pub path: String,
    /// The relation type from the edge that connects this file to its dependent.
    pub relation: String,
    /// Distance from the start file (1 = direct dependent, N = N-hop dependent).
    pub depth: u32,
}

/// Result of an impact analysis.
#[derive(Debug, Clone, Serialize)]
pub struct ImpactResult {
    pub files: Vec<ImpactFile>,
}

/// Compute transitive impact: find all files that depend on `start_path`,
/// directly or transitively, up to `max_depth` hops.
///
/// The DuckDB `edges` table has columns `"from"` (source file), `"to"` (dependency),
/// `rel` (relation type). Impact analysis finds files whose `"from"` appears as
/// a dependent of `start_path` or its transitive dependents.
///
/// Uses BFS with a visited set to protect against circular imports.
/// Returns files ordered by discovery (BFS order) — direct dependents first,
/// then 2-hop, etc.
pub fn compute_impact(conn: &Connection, start_path: &str, max_depth: u32) -> GraphResult<Vec<ImpactFile>> {
    if max_depth == 0 {
        return Ok(Vec::new());
    }

    let mut results: Vec<ImpactFile> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start_path.to_string());

    let mut queue: VecDeque<(String, u32)> = VecDeque::new();
    queue.push_back((start_path.to_string(), 0));

    let mut stmt = conn.prepare(r#"SELECT "from", rel FROM edges WHERE "to" = ?"#)?;

    while let Some((path, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        let rows = stmt.query_map(params![path], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows {
            let (from, rel) = row?;
            if visited.insert(from.clone()) {
                results.push(ImpactFile {
                    path: from.clone(),
                    relation: rel,
                    depth: depth + 1,
                });
                queue.push_back((from, depth + 1));
            }
        }
    }

    Ok(results)
}
