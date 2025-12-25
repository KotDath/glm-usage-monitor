//! GLM Usage Monitor - Realtime GLM Coding Plan usage monitor with TUI

#![allow(clippy::doc_markdown)]

mod app;
mod api;
mod config;
mod models;
mod terminal;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;
use std::time::Duration;

/// GLM Usage Monitor - Realtime GLM Coding Plan usage monitor with TUI
#[derive(Debug, Parser)]
struct Cli {
    /// Override refresh interval in seconds (default: from ENV or 300)
    #[arg(short, long)]
    refresh_sec: Option<u64>,

    /// Override HTTP timeout in seconds (default: from ENV or 20)
    #[arg(short, long)]
    timeout_sec: Option<u64>,

    /// Tick rate for the UI in milliseconds (default: 250)
    #[arg(long, default_value_t = 250)]
    tick_rate: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let mut config = config::Config::load()
        .context("Failed to load configuration. Please ensure ANTHROPIC_BASE_URL and ANTHROPIC_AUTH_TOKEN are set, or create a config file at ~/.config/glm-usage-monitor/config.toml")?;

    // Apply CLI overrides
    if let Some(refresh) = cli.refresh_sec {
        config.refresh_sec = refresh;
    }
    if let Some(timeout) = cli.timeout_sec {
        config.http_timeout_sec = timeout;
    }

    // Create application
    let mut app = app::App::new(config)
        .context("Failed to initialize application")?;

    // Run initial data fetch
    app.refresh_data().await;

    // Run TUI
    let tick_rate = Duration::from_millis(cli.tick_rate);
    terminal::run(&mut app, tick_rate)
        .await
        .context("Failed to run TUI")?;

    Ok(())
}
