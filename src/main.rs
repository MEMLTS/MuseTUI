use crate::tui::app::run;

mod crypto;
mod api;
mod tui;
mod player;
mod utils;
mod config;

fn main() -> anyhow::Result<()> {
    config::AppConfig::load()?;
    color_eyre::install().expect("ERROR");
    let terminal = ratatui::init();
    run(terminal)?;
    ratatui::restore();
    Ok(())
}
