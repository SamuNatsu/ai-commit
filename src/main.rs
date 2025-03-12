mod api;
mod args;
mod utils;

use std::{
    env,
    io::{self, Write},
};

use anyhow::{Context, Result, bail};
use api::Api;
use args::Args;
use clap::Parser;
use futures_util::StreamExt;

const DEFAULT_FILTERS: &'static str = include_str!("./includes/default_filters.txt");
const PROMPT_TEMPLATE: &'static str = include_str!("./includes/prompt_template.txt");

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Parse .env file
    if let Some(d) = &args.dotenv {
        dotenv::from_filename(format!(".env.{d}"))?;
    } else {
        dotenv::dotenv().ok();
    }

    // Check working directory is a repository
    if !utils::is_git_repo().await? {
        bail!("current working directory is not a git repository");
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
    let diff = utils::filter_diff(diff, &filter)?;
    let diff = diff.trim();

    // Check final diff is empty
    if diff.is_empty() {
        bail!("there is no significant changes to be committed");
    }

    // Create additional messages
    let mut commit_type = String::new();
    if let Some(t) = &args.commit_type {
        commit_type = format!(
            "According to my demand, the commit type must be '{}'.\n\n\n",
            t
        );
    }

    let mut prompt = String::new();
    if let Some(p) = &args.prompt {
        prompt = format!(
            "Here are some additional message for you to writer a better git commit message:\n\n{}\n\n\n",
            p
        );
    }

    // Create prompt
    let prompt = PROMPT_TEMPLATE
        .replace("<|FILTER|>", &filter)
        .replace("<|COMMIT_TYPE|>", &commit_type)
        .replace("<|PROMPT|>", &prompt)
        .replace("<|DIFF|>", &diff);
    let prompt = prompt.trim();

    if args.verbose {
        println!("{}\n", console::style("Prompt:").italic().cyan());
        println!("{}\n", console::style(&prompt).italic().bright().black());
    }

    // Create API
    let endpoint = env::var("AI_COMMIT_ENDPOINT")
        .with_context(|| "try to get environment variable `AI_COMMIT_ENDPOINT`")?;
    let api_key = env::var("AI_COMMIT_API_KEY")
        .with_context(|| "try to get environment variable `AI_COMMIT_API_KEY`")?;
    let model = env::var("AI_COMMIT_MODEL")
        .with_context(|| "try to get environment variable `AI_COMMIT_MODEL`")?;
    let api = Api::new(endpoint.trim(), api_key.trim(), model.trim());

    if args.verbose {
        println!(
            "{}",
            console::style(format!("Endpoint: {endpoint}\nModel: {model}\n"))
                .italic()
                .cyan()
        );
    }

    // Start generating
    println!(
        "{}\n",
        console::style("Generating...").bold().bright().cyan()
    );

    let mut r = api
        .gen_completion(prompt)
        .await
        .with_context(|| "try to generate completion")?;
    let mut is_reason = false;
    let mut reason_end = false;

    while let Some(msg) = r.next().await {
        let msg = msg?;

        // Print usage
        if let Some(usage) = msg.usage {
            println!("\n");
            println!(
                "{}",
                console::style(format!(
                    "Prompt Tokens: {}\nCompletion Tokens: {}\nTotal Tokens: {}",
                    usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
                ))
                .bright()
                .black()
            );

            break;
        }

        // Print unexpected finish
        if let Some(finish) = &msg.choices[0].finish_reason {
            if finish != "stop" {
                bail!("unexpected generating end: {finish}");
            }
        }

        // Check is reason
        if !is_reason && msg.choices[0].delta.reasoning_content.is_some() {
            is_reason = true;
            println!(
                "{}\n",
                console::style("Reasoning model detected, you can use `-v` or `--verbose` flag to see the reasoning contents")
                    .bold()
                    .bright()
                    .cyan()
            );
        }

        // Check reason end
        if !reason_end && msg.choices[0].delta.content.is_some() {
            reason_end = true;

            if is_reason && args.verbose {
                println!("\n");
            }
        }

        // Print contents
        if reason_end {
            print!("{}", msg.choices[0].delta.content.clone().unwrap());
            io::stdout().flush()?;
        } else if is_reason && args.verbose {
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
