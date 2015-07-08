//! Literexp metadata, parsing, etc.
//! Useful for people who wanna do their own thang, I guess.

#![feature(subslice_offset)]

// Lint fairly aggressively, manually allowing where necessary
#![warn(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]

#![feature(plugin)]
#![plugin(plex)]

mod ast;
mod document;
mod error;
mod expression;
mod lexer;
mod parser;

// That said, re-export the important stuff anyways
pub use document::{ Document, Strategy };
pub use error::{ Error, Result };
