mod commands;

use clap::{Parser, Subcommand};

use commands::{graph, init, meta, serve};

/// WarpFS command-line interface.
#[derive(Parser)]
#[command(name = "warpfs", about = "WarpFS CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a WarpFS project in the current directory.
    Init(InitArgs),
    /// Show WarpFS extended attributes for a file.
    Meta(MetaArgs),
    /// Dependency-graph discovery and statistics.
    #[command(subcommand)]
    Graph(GraphCommand),
    /// Run a WarpFS server (MCP stub).
    Serve(ServeArgs),
}

#[derive(clap::Args)]
struct InitArgs {}

#[derive(clap::Args)]
struct MetaArgs {
    /// The file whose WarpFS metadata to inspect or set.
    path: String,

    /// Set a WarpFS extended attribute (e.g. `user.vfs.feature`).
    #[arg(long)]
    set: Option<String>,

    /// Value for --set. Accepts literal `\n` for multiline values.
    #[arg(long, requires = "set")]
    value: Option<String>,
}

#[derive(Subcommand)]
enum GraphCommand {
    /// Walk the current directory, parse Go imports, and populate the graph.
    Discover,
    /// Print summary statistics from the discovered dependency graph.
    Stats,
    /// Query graph edges for a specific file.
    Related(RelatedArgs),
}

#[derive(clap::Args)]
struct RelatedArgs {
    /// The file whose graph edges to query.
    path: String,

    /// Filter edges by relation type (e.g., "imports", "calls").
    #[arg(long)]
    relation: Option<String>,
}

#[derive(clap::Args)]
struct ServeArgs {
    /// Run as an MCP server.
    #[arg(long)]
    mcp: bool,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init(_) => init::run(),
        Commands::Meta(args) => meta::run(&args.path, args.set.as_deref(), args.value.as_deref()),
        Commands::Graph(GraphCommand::Discover) => graph::run_discover(),
        Commands::Graph(GraphCommand::Stats) => graph::run_stats(),
        Commands::Graph(GraphCommand::Related(args)) => graph::run_related(&args.path, args.relation.as_deref()),
        Commands::Serve(args) => serve::run(args.mcp),
    };

    if let Err(e) = result {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
