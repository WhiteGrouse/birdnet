#![feature(vec_into_raw_parts)]
#![allow(dead_code)]

#[macro_use]
extern crate birdnet_derive;

pub mod utils;
pub mod types;
pub mod socket;
mod abort_when_drop;
pub use abort_when_drop::AbortWhenDrop;
pub mod protocol;
mod settings;
pub use settings::PeerSettings;
pub mod service;
