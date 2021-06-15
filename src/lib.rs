#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod error;
pub mod resources;

use std::result;

//TODO: Use anyhow instead
pub type Result<T> = result::Result<T, error::Error>;