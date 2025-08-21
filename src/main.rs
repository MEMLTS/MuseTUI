mod crypto;
mod api;
mod tui;
mod player;
mod utils;
mod config;

fn main() {
    config::AppConfig::load().unwrap();
    println!("Hello, world!");
}
