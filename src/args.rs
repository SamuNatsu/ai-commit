use clap::Parser;

/// A Command Line Utility for AI Generating Git Commit Message
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Show verbose message
    #[arg(short, long)]
    pub verbose: bool,

    /// Dotenv profile name
    #[arg(short, long)]
    pub dotenv: Option<String>,

    /// Force using the given commit type
    #[arg(short = 't', long)]
    pub commit_type: Option<String>,

    /// Force using the given commit scope
    #[arg(short = 's', long)]
    pub commit_scope: Option<String>,

    /// Additional prompt message
    #[arg(short, long)]
    pub prompt: Option<String>,
}
