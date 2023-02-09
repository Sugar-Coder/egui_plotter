#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod reader;
mod chart;
pub use app::TemplateApp;
pub use chart::ChartsDemo;