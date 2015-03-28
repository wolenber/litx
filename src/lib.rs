//! Literexp metadata, parsing, etc.
//! Useful for people who wanna do their own thang, I guess.

#![feature(box_syntax)]
#![feature(collections)]
#![feature(core)]
#![feature(plugin)]

// Lint fairly aggressively, manually allowing where necessary
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(dead_code)]

#![plugin(rustlex)]

#[allow(plugin_as_library)]
    extern crate rustlex;

// Every module is kept public right now
// I don't see any reason to close off access yet;
// I want this to be usable as a library or a program.
pub mod doc;
pub mod error;
pub mod lex;
pub mod parse;
pub mod preprocess;
pub mod render;
pub mod syntax;
pub mod strategy;
pub mod token;

// That said, re-export the important stuff anyways
pub use doc::Document;
pub use error::{ Error, Result };
pub use render::RenderSystem;
pub use render::PdfRenderer;
pub use strategy::Strategy;
