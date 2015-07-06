//! Literexp metadata, parsing, etc.
//! Useful for people who wanna do their own thang, I guess.

// Lint fairly aggressively, manually allowing where necessary
#![warn(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]

#![feature(plugin)]
#![plugin(plex)]

mod error;
mod lexer;

// That said, re-export the important stuff anyways
pub use error::{ Error, Result };