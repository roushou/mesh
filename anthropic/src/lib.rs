#![doc = include_str!("../README.md")]

pub mod client;
pub mod completion;
pub mod config;
pub mod error;
pub mod models;

pub mod prelude {
    pub use crate::{client, completion, config, error, models};
}
