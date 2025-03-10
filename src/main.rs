mod api;
mod utils;

use std::{
    env,
    io::{self, Write},
    process,
};

use anyhow::{Context, Result};
use api::Api;
use futures_util::StreamExt;

const DEFAULT_FILTERS: &'static str = include_str!("./includes/default_filters.txt");

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Check working directory is a repository
    if !utils::is_git_repo().await? {
        eprintln!("This is not a git repository");
        process::exit(1);
    }

    // Load filter
    let filter = DEFAULT_FILTERS
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0 && !line.starts_with("#"))
        .fold(String::new(), |prev, cur| format!("{prev}|{cur}"));
    let filter = env::var("AI_COMMIT_FILTER").unwrap_or(filter[1..].to_owned());

    // Get filtered diff
    let diff = utils::get_git_diff().await?;
    let diff = utils::filter_diff(diff, filter)?;
    let diff = diff.trim();

    // Check final diff is empty
    if diff.is_empty() {
        eprintln!("No significant changes to commit");
        process::exit(1);
    }

    // Create API
    let endpoint = env::var("AI_COMMIT_ENDPOINT")
        .with_context(|| "try to get environment variable `AI_COMMIT_ENDPOINT`")?;
    let api_key = env::var("AI_COMMIT_API_KEY")
        .with_context(|| "try to get environment variable `AI_COMMIT_API_KEY`")?;
    let model = env::var("AI_COMMIT_MODEL")
        .with_context(|| "try to get environment variable `AI_COMMIT_MODEL`")?;
    let show_reason = env::var("AI_COMMIT_SHOW_REASON").unwrap_or("1".to_owned()) == "1";
    let api = Api::new(endpoint.trim(), api_key.trim(), model.trim());

    // Start generating
    println!("{}\n", console::style("Generating...").bright().cyan());

    let mut r = api
        .gen_completion(diff)
        .await
        .with_context(|| "try to generate completion")?;
    let mut reason_end = false;

    while let Some(msg) = r.next().await {
        let msg = msg?;

        // Print usage
        if let Some(usage) = msg.usage {
            println!("\n");
            println!(
                "{}",
                console::style(format!("Prompt Tokens: {}", usage.prompt_tokens))
                    .bright()
                    .black()
            );
            println!(
                "{}",
                console::style(format!("Completion Tokens: {}", usage.completion_tokens))
                    .bright()
                    .black()
            );
            println!(
                "{}",
                console::style(format!("Total Tokens: {}", usage.total_tokens))
                    .bright()
                    .black()
            );

            break;
        }

        // Print unexpected finish
        if let Some(finish) = &msg.choices[0].finish_reason {
            if finish != "stop" {
                eprintln!("Unexpected generating ended: {finish}");
                process::exit(1);
            }
        }

        // Check reason end
        if !reason_end && msg.choices[0].delta.content.is_some() {
            reason_end = true;

            if show_reason {
                println!("\n");
            }
        }

        // Print contents
        if reason_end {
            print!("{}", msg.choices[0].delta.content.clone().unwrap());
            io::stdout().flush()?;
        } else if show_reason {
            print!(
                "{}",
                console::style(msg.choices[0].delta.reasoning_content.clone().unwrap())
                    .italic()
                    .bright()
                    .black()
            );
            io::stdout().flush()?;
        }
    }

    // Success
    Ok(())
}
