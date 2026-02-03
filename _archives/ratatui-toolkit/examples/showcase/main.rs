//! Showcase Demo - Demonstrates all ratatui-toolkit components
//!
//! Run with: cargo run --example showcase
//! Or: just dev

mod app;
mod constants;
mod demo_tab;
mod handlers;
mod helpers;
mod render;

fn main() -> std::io::Result<()> {
    handlers::main_loop_handler::run_main_loop()
}
