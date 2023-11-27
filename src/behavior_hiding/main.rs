use clap::{Parser, Subcommand, Args};
mod interface;

#[derive(Parser)]
#[clap(author = "James Lako", version = "1.0", about = "Behavior Hiding Module")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitArgs),
    Clone(CloneArgs),
    Commit(CommitArgs),
}

#[derive(Args)]
struct InitArgs {
    path: String,
}

#[derive(Args)]
struct CloneArgs {
    source: String,
    destination: String,
}

#[derive(Args)]
struct CommitArgs {
    repo_path: String,
    message: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => {
            interface::interface::init(&args.path);
            println!("Repository initialized at {}", args.path);
        },
        Commands::Clone(args) => {
            interface::interface::clone(&args.source, &args.destination);
            println!("Repository cloned from {} to {}", args.source, args.destination);
        },
        Commands::Commit(args) => {
            interface::interface::commit(&args.repo_path, &args.message);
            println!("Changes committed to {}: {}", args.repo_path, args.message);
        },
    }
}
