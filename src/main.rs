#[path = "erro.rs"]
pub mod errors;

mod analysis;
mod app;
mod config;
mod db;
mod models;
mod questions;
mod reflection;
mod types;
mod ui;

fn main() -> errors::Result<()> {
    let mut app = app::App::initialize()?;
    app.run()
}
