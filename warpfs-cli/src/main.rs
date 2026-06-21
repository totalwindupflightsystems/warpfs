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
    /// The file whose WarpFS metadata to inspect.
    path: String,
}

#[derive(Subcommand)]
enum GraphCommand {
    /// Walk the current directory, parse Go imports, and populate the graph.
    Discover,
    /// Print summary statistics from the discovered dependency graph.
    Stats,
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
        Commands::Meta(args) => meta::run(&args.path),
        Commands::Graph(GraphCommand::Discover) => graph::run_discover(),
        Commands::Graph(GraphCommand::Stats) => graph::run_stats(),
        Commands::Serve(args) => serve::run(args.mcp),
    };

    if let Err(e) = result {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}
