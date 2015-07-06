//! Whole document structure, formatting, etc.

use error::Error;
use error::Result;
use lex::Lexer;
use parse;
use preprocess;
use render::RenderSystem;
use strategy::Strategy;
use syntax;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

/// Completed document, and the final in memory representation.
///
/// At this point, a Document should be ready to be run through a renderer.
#[derive(Debug)]
pub struct Document {
    strategy: Strategy
}

impl Document {
    /// Create a document from an expression
    ///
    /// At this stage, it is too late to preprocess.
    /// As such, we don't need a working directory.
    pub fn new(expr: syntax::Node) -> Result<Document> {
        // HACK This is gonna be ugly, inefficient, and terrible for a while.
        let strategy = expr.property("strategy").unwrap().as_text().unwrap();
        println!("Document strategy: {}", strategy);
        Err(Error::Unimplemented(file!(), line!()))
    }

    /// Create a document from a source file.
    pub fn new_from_file(file: &Path) -> Result<Document> {
        let reader = BufReader::new(try!(File::open(file)));
        let dir = file.parent().unwrap(); // Should be safe? File already found, should have a folder.
        Document::new_from_reader(reader, Some(dir))
    }

    /// Create a Document from a source string, and a working directory.
    ///
    /// The working directory is necessary because of possible preprocessor commands.
    pub fn new_from_string(string: &str, working_dir: Option<&Path>) -> Result<Document> {
        let reader = BufReader::new(string.as_bytes());
        Document::new_from_reader(reader, working_dir)
    }

    /// Create a Document from any buf reader and working directory.
    pub fn new_from_reader<R: BufRead>(reader: R, working_dir: Option<&Path>) -> Result<Document> {
        let source = preprocess::preprocess(reader, working_dir);
        println!("{}", source);
        let mut tokens = try!(Lexer::new_from_string(source));
        let ast = match parse::next_node(&mut tokens) {
            Err(e) => return Err(e),
            Ok(None) => return Err(Error::ParseFailure),
            Ok(Some(node)) => node,
        };
        Document::new(ast)
    }

    /// Alternate syntax, allowing `document.render(renderer)` instead of `renderer.render(document)`
    pub fn render(&self, renderer: &RenderSystem) -> Result<()>{
        renderer.render(self)
    }
}
