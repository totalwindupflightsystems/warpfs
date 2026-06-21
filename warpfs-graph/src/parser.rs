//! AST parsing with tree-sitter for Go source files.
//!
//! Uses the tree-sitter-go grammar to parse Go files and extract import declarations.

use tree_sitter::{Node, Parser as TsParser};
use warpfs_metadata::inventory::Edge;

use crate::error::{GraphError, GraphResult};

/// Parses Go source and extracts import edges.
///
/// Construct once with [`Parser::new`] and reuse across files by calling
/// [`Parser::parse_imports`] for each source file.
pub struct Parser {
    parser: TsParser,
}

impl Parser {
    /// Create a new [`Parser`] with the Go grammar loaded.
    pub fn new() -> GraphResult<Self> {
        let mut parser = TsParser::new();
        parser.set_language(&tree_sitter_go::LANGUAGE.into())?;
        Ok(Parser { parser })
    }

    /// Parse a Go source file and return the import edges it declares.
    ///
    /// Each import declaration produces one [`Edge`] with `rel = "imports"`.
    /// The `from` field is `file_path`, and `to` is the classified dependency
    /// target:
    /// - Standard library imports (no `.` in the path) are prefixed `"std:"`,
    ///   e.g. `"fmt"` becomes `"std:fmt"`.
    /// - Third-party imports (a `.` in the path, per Go module convention)
    ///   are prefixed `"pkg:"`, e.g. `"github.com/foo/bar"` becomes
    ///   `"pkg:github.com/foo/bar"`.
    pub fn parse_imports(&mut self, file_path: &str, source: &str) -> GraphResult<Vec<Edge>> {
        let tree = self
            .parser
            .parse(source.as_bytes(), None)
            .ok_or_else(|| GraphError::Other("tree-sitter produced no parse tree".to_string()))?;

        let mut paths: Vec<String> = Vec::new();
        collect_import_paths(tree.root_node(), source.as_bytes(), &mut paths);

        let edges = paths
            .into_iter()
            .map(|path| Edge {
                from: file_path.to_string(),
                to: classify(&path),
                rel: "imports".to_string(),
            })
            .collect();
        Ok(edges)
    }
}

/// Recursively walk the tree, appending the path string of every `import_spec`.
///
/// We recurse until we reach `import_spec` nodes, then read their
/// `interpreted_string_literal` child (the quoted import path) and strip the
/// surrounding quotes. Child nodes are collected into a [`Vec`] before
/// recursing so the [`tree_sitter::TreeCursor`] borrow is released between
/// visits.
fn collect_import_paths(node: Node, source: &[u8], out: &mut Vec<String>) {
    if node.kind() == "import_spec" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "interpreted_string_literal" {
                if let Ok(text) = child.utf8_text(source) {
                    let cleaned = text.trim_matches('"');
                    out.push(cleaned.to_string());
                }
            }
        }
        return;
    }

    let children: Vec<Node> = {
        let mut cursor = node.walk();
        node.children(&mut cursor).collect()
    };
    for child in children {
        collect_import_paths(child, source, out);
    }
}

/// Classify an import path as standard library (`std:`) or third-party (`pkg:`).
///
/// Go convention: the first path segment of a third-party module always
/// contains a dot (the module's domain), while standard library paths never
/// contain a dot. Using "contains a dot anywhere" matches this convention for
/// all realistic Go imports.
fn classify(path: &str) -> String {
    if path.contains('.') {
        format!("pkg:{path}")
    } else {
        format!("std:{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_std_and_pkg() {
        assert_eq!(classify("fmt"), "std:fmt");
        assert_eq!(classify("net/http"), "std:net/http");
        assert_eq!(
            classify("github.com/gin-gonic/gin"),
            "pkg:github.com/gin-gonic/gin"
        );
        assert_eq!(classify("golang.org/x/net"), "pkg:golang.org/x/net");
    }
}
