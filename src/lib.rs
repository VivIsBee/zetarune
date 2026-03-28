#![cfg_attr(target_os = "horizon", no_std)]
//! Game engine for deltarune-likes.

extern crate alloc;

pub mod ctx;
pub mod objs;
pub mod rt;
#[macro_use]
pub mod log;
pub mod components;
pub mod resources;

pub use zetarune_proc::compressed_sprites;

pub use hashbrown::{HashMap, HashSet};

#[cfg(target_os = "horizon")]
mod switch_impl;
