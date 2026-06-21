//! `warpfs graph discover`, `warpfs graph stats`, and `warpfs graph impact`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use warpfs_graph::{impact, GraphDB, ImpactResult, Language, Parser};
use warpfs_metadata::inventory::{self, Edge};

/// Directory names to skip when walking for source files.
const SKIP_DIRS: &[&str] = &[
    "target",       // Rust build output
    "node_modules", // JavaScript/TypeScript
    "vendor",       // Go / PHP
    "__pycache__",  // Python cache
    ".venv",        // Python virtualenv
];

/// Walk the current directory for source files in all supported languages,
/// parse their imports, and write the resulting edges to both
/// `.vfs/graph/edges.jsonl` and `.vfs/graph/graph.db`.
pub fn run_discover() -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine the current directory")?;

    // Collect every source file under the current directory.
    let mut source_files = Vec::new();
    collect_source_files(&cwd, &mut source_files)
        .context("failed to walk directory tree for source files")?;

    if source_files.is_empty() {
        println!("No supported source files found. Supported extensions: {}",
            Language::all_extensions().join(", "));
        return Ok(());
    }

    // Group files by language so we can reuse one parser per language.
    let mut by_lang: std::collections::BTreeMap<Language, Vec<PathBuf>> =
        std::collections::BTreeMap::new();
    for file in &source_files {
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            if let Some(lang) = Language::from_extension(ext) {
                by_lang.entry(lang).or_default().push(file.clone());
            }
        }
    }

    let mut all_edges: Vec<Edge> = Vec::new();
    let mut unique_sources: HashSet<String> = HashSet::new();

    for (language, files) in &by_lang {
        if files.is_empty() { continue; }
        let lang_name = format!("{:?}", language);
        let mut parser = Parser::for_language(*language)
            .with_context(|| format!("failed to initialize tree-sitter {lang_name} parser"))?;

        for file in files {
            let source = match std::fs::read_to_string(file) {
                Ok(s) => s,
                Err(_) => continue,
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
    }

    // Infer `tested_by` and `tests` edges from filename conventions.\n    let test_edges = discover_test_associations(&source_files, &cwd);\n    all_edges.extend(test_edges);\n\n    // Persist edges to the JSONL inventory file.
    let edges_jsonl = cwd.join(".vfs").join("graph").join("edges.jsonl");
    inventory::append_edges_deduped(&edges_jsonl, &all_edges)
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
    let langs = by_lang.len();
    println!("Discovered {n} edges across {m} files ({langs} languages)");
    Ok(())
}

/// Query all edges where `from = path`, optionally filtered by relation type.
///
/// Exits with code 1 and a "not found in graph" message when the path does not
/// appear in the `edges` table at all (neither as `from` nor `to`).
pub fn run_related(path: &str, relation: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine the current directory")?;
    let graph_db = cwd.join(".vfs").join("graph").join("graph.db");

    if !graph_db.exists() {
        anyhow::bail!("No graph data. Run `warpfs graph discover` first.");
    }

    let graph_db_str = graph_db.to_str().unwrap_or(".vfs/graph/graph.db");
    let graph = GraphDB::open(graph_db_str).context("failed to open DuckDB graph database")?;

    // Check whether the file exists in the graph at all.
    if !graph
        .file_in_graph(path)
        .context("failed to query graph for file existence")?
    {
        anyhow::bail!("not found in graph");
    }

    let edges = graph
        .related(path, relation)
        .context("failed to query related edges")?;

    if edges.is_empty() && relation.is_some() {
        println!(
            "No edges found for '{}' with relation filter '{}'.",
            path,
            relation.unwrap()
        );
        return Ok(());
    }

    // Print edges in a readable format.
    if edges.is_empty() {
        println!("No outgoing edges from '{}'.", path);
    } else {
        for edge in &edges {
            println!("{}  →  {}  ({})", edge.from, edge.to, edge.rel);
        }
    }

    Ok(())
}

/// Compute transitive impact: find all files that depend on `path`, directly
/// or transitively, up to `max_depth` hops.
///
/// When `format` is `"json"`, prints the result as pretty-printed JSON.
/// Otherwise prints each dependent file in human-readable text.
pub fn run_impact(path: &str, max_depth: u32, format: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine the current directory")?;
    let graph_db = cwd.join(".vfs").join("graph").join("graph.db");

    if !graph_db.exists() {
        anyhow::bail!("No graph data. Run `warpfs graph discover` first.");
    }

    let graph_db_str = graph_db.to_str().unwrap_or(".vfs/graph/graph.db");
    let graph = GraphDB::open(graph_db_str).context("failed to open DuckDB graph database")?;

    // Check whether the file exists in the graph at all.
    if !graph
        .file_in_graph(path)
        .context("failed to query graph for file existence")?
    {
        anyhow::bail!("not found in graph");
    }

    let results = impact::compute_impact(graph.conn(), path, max_depth)
        .context("failed to compute impact")?;

    match format {
        Some("json") => {
            let result = ImpactResult { files: results };
            let json = warpfs_graph::serde_json::to_string_pretty(&result)
                .context("failed to serialize impact results as JSON")?;
            println!("{}", json);
        }
        _ => {
            if results.is_empty() {
                println!("No dependents found for '{}'.", path);
            } else {
                for file in &results {
                    println!("{}  ←  {}  (depth: {})", file.path, file.relation, file.depth);
                }
            }
        }
    }

    Ok(())
}

/// Print summary statistics from the discovered dependency graph.
pub fn run_stats() -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine the current directory")?;
    let graph_db = cwd.join(".vfs").join("graph").join("graph.db");

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

/// Infer `tested_by` and `tests` edges from common filename conventions across\n/// all 9 supported languages.\n///\n/// - `*_test.go` → tested_by → `*.go` (and reverse: `*.go` → tests → `*_test.go`)\n/// - `test_*.py` → tested_by → `*.py`\n/// - `*.test.ts` → tested_by → `*.ts`\n/// - `*.spec.ts` → tested_by → `*.ts`\n/// - `*_test.rs` → tested_by → `*.rs`\n/// - `*Test.java` → tested_by → `*.java`\n/// - `test_*.c` → tested_by → `*.c`\n/// - `*_test.cpp` → tested_by → `*.cpp`\n/// - `*_test.rb` → tested_by → `*.rb`\nfn discover_test_associations(source_files: &[PathBuf], cwd: &Path) -> Vec<Edge> {\n    let mut edges = Vec::new();\n    let stem_set: HashSet<String> = source_files\n        .iter()\n        .filter_map(|p| {\n            let rel = p.strip_prefix(cwd).unwrap_or(p);\n            Some(rel.to_string_lossy().into_owned())\n        })\n        .collect();\n\n    for file in source_files {\n        let rel = file.strip_prefix(cwd).unwrap_or(file);\n        let file_name = rel\n            .file_name()\n            .and_then(|n| n.to_str())\n            .unwrap_or(\"\");\n        let file_str = rel.to_string_lossy();\n\n        // Check if this is a test file → generate tested_by edge\n        if let Some(source_stem) = test_to_source(file_name) {\n            let parent = rel.parent().unwrap_or(Path::new(\"\"));\n            let source_path = parent.join(&source_stem);\n            let source_str = source_path.to_string_lossy().into_owned();\n            if stem_set.contains(&source_str) || file_name == &source_stem {\n                edges.push(Edge {\n                    from: file_str.into_owned(),\n                    to: source_str,\n                    rel: \"tested_by\".to_string(),\n                });\n            }\n        }\n\n        // Check if this is a source file that has a corresponding test file → tests edge\n        for test_stem in source_to_test_patterns(file_name) {\n            let parent = rel.parent().unwrap_or(Path::new(\"\"));\n            let test_path = parent.join(&test_stem);\n            let test_str = test_path.to_string_lossy().into_owned();\n            if stem_set.contains(&test_str) {\n                edges.push(Edge {\n                    from: file_str.clone().into_owned(),\n                    to: test_str,\n                    rel: \"tests\".to_string(),\n                });\n            }\n        }\n    }\n\n    edges\n}\n\n/// If `file_name` is a test file, return the source file stem it tests.\nfn test_to_source(name: &str) -> Option<String> {\n    if let Some(stem) = name.strip_suffix(\"_test.go\") {\n        Some(format!(\"{stem}.go\"))\n    } else if let Some(stem) = name.strip_suffix(\"_test.rs\") {\n        Some(format!(\"{stem}.rs\"))\n    } else if let Some(stem) = name.strip_suffix(\"_test.cpp\") {\n        Some(format!(\"{stem}.cpp\"))\n    } else if let Some(stem) = name.strip_suffix(\"_test.rb\") {\n        Some(format!(\"{stem}.rb\"))\n    } else if let Some(stem) = name.strip_prefix(\"test_\") {\n        if stem.ends_with(\".py\") {\n            Some(stem.to_string())\n        } else if stem.ends_with(\".c\") {\n            Some(stem.to_string())\n        } else {\n            None\n        }\n    } else if let Some(stem) = name.strip_suffix(\".test.ts\") {\n        Some(format!(\"{stem}.ts\"))\n    } else if let Some(stem) = name.strip_suffix(\".spec.ts\") {\n        Some(format!(\"{stem}.ts\"))\n    } else if let Some(stem) = name.strip_suffix(\"Test.java\") {\n        Some(format!(\"{stem}.java\"))\n    } else {\n        None\n    }\n}\n\n/// Return possible test file names for a given source file.\nfn source_to_test_patterns(name: &str) -> Vec<String> {\n    let mut patterns = Vec::new();\n    if let Some(stem) = name.strip_suffix(\".go\") {\n        patterns.push(format!(\"{stem}_test.go\"));\n    } else if let Some(stem) = name.strip_suffix(\".py\") {\n        patterns.push(format!(\"test_{stem}.py\"));\n    } else if let Some(stem) = name.strip_suffix(\".ts\") {\n        patterns.push(format!(\"{stem}.test.ts\"));\n        patterns.push(format!(\"{stem}.spec.ts\"));\n    } else if let Some(stem) = name.strip_suffix(\".rs\") {\n        patterns.push(format!(\"{stem}_test.rs\"));\n    } else if let Some(stem) = name.strip_suffix(\".java\") {\n        patterns.push(format!(\"{stem}Test.java\"));\n    } else if let Some(stem) = name.strip_suffix(\".c\") {\n        patterns.push(format!(\"test_{stem}.c\"));\n    } else if let Some(stem) = name.strip_suffix(\".cpp\") {\n        patterns.push(format!(\"{stem}_test.cpp\"));\n    } else if let Some(stem) = name.strip_suffix(\".rb\") {\n        patterns.push(format!(\"{stem}_test.rb\"));\n    }\n    patterns\n}\n\n/// Recursively collect source files for all supported languages.
fn collect_source_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') || SKIP_DIRS.contains(&name_str.as_ref()) {
            continue;
        }

        let ft = entry.file_type()?;
        if ft.is_dir() {
            collect_source_files(&path, out)?;
        } else if ft.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if Language::from_extension(ext).is_some() {
                    out.push(path);
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Rule engine — manifest-driven SQL queries against the graph
// ---------------------------------------------------------------------------

/// Default manifest paths (relative to CWD).
const MANIFEST_PATHS: &[&str] = &["manifest.yaml", ".vfs/manifest.yaml"];

/// Load the manifest from the first available path.
fn load_manifest() -> Result<warpfs_core::manifest::Manifest> {
    for path in MANIFEST_PATHS {
        if std::path::Path::new(path).exists() {
            return Ok(warpfs_core::manifest::Manifest::from_file(path)?);
        }
    }
    anyhow::bail!(
        "No manifest found. Create a manifest.yaml or .vfs/manifest.yaml file with `warpfs init`."
    );
}

/// `warpfs graph rule-list` — print all rules from the manifest.
pub fn run_rule_list() -> Result<()> {
    let manifest = load_manifest()?;

    if manifest.rules.is_empty() {
        println!("No rules defined in the manifest.");
        return Ok(());
    }

    println!("Rules defined in manifest:");
    for rule in &manifest.rules {
        println!("  {} — {}", rule.name, rule.description);
    }
    Ok(())
}

/// `warpfs graph rule-check <name>` — execute a named rule against the graph.
pub fn run_rule_check(name: &str) -> Result<()> {
    let cwd = std::env::current_dir().context("failed to determine the current directory")?;
    let graph_db = cwd.join(".vfs").join("graph").join("graph.db");

    if !graph_db.exists() {
        anyhow::bail!("No graph data. Run `warpfs graph discover` first.");
    }

    let manifest = load_manifest()?;

    let query_rule = manifest
        .rules
        .iter()
        .find(|r| r.name == name)
        .ok_or_else(|| {
            let available: Vec<&str> = manifest.rules.iter().map(|r| r.name.as_str()).collect();
            anyhow::anyhow!(
                "Rule '{}' not found in manifest. Available: {}",
                name,
                available.join(", ")
            )
        })?;

    let rule = warpfs_graph::Rule {
        name: query_rule.name.clone(),
        description: query_rule.description.clone(),
        query: query_rule.query.clone(),
    };

    let graph_db_str = graph_db.to_str().unwrap_or(".vfs/graph/graph.db");
    let graph = GraphDB::open(graph_db_str).context("failed to open DuckDB graph database")?;

    match warpfs_graph::RuleEngine::check(graph.conn(), &rule) {
        Ok(result) => {
            if result.matches.is_empty() {
                println!("No matches for rule '{}'.", name);
            } else {
                println!("Rule '{}' — {} match(es):", name, result.total);
                for row in &result.matches {
                    println!("  {}", row.join(" | "));
                }
            }
            Ok(())
        }
        Err(err) => {
            // Return structured error — never panic.
            anyhow::bail!("Rule '{}' failed: {}", err.rule, err.error);
        }
    }
}
