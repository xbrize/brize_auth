#![forbid(unsafe_code)]

mod domain;
pub use domain::config;

mod application;
pub use application::interface;

mod infrastructure;
pub use infrastructure::gateway::*;

pub mod auth;
