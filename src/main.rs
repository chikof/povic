use color_eyre::Result;

use crate::app::App;

mod action;
mod app;
mod components;
mod config;
mod errors;
mod logging;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let config = crate::config::init()?;

    crate::errors::init()?;
    crate::logging::init(&config)?;

    let mut app = App::new(config)?;
    app.run()
        .await?;

    Ok(())
}
