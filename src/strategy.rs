//! Document strategy

use error::Error;
use error::Result;
use lex::Lexer;
use parse;
use preprocess;
use syntax;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

/// A document strategy is a template used for handling certain features of a document.
///
/// The strategy contains default formatting information, as well as meta-fields.
#[derive(Debug)]
pub struct Strategy {
    /// The name of the strategy.
    name: String,

    /// Meta fields of the strategy, such as author's name.
    ///
    /// Theoretically, these fields are very small changes, and don't effect formatting
    meta: Vec<Field>,

    /// Fields of the strategy, such as page size and works-cited
    ///
    /// Theoretically, these are the larger fields which effect formatting more strongly.
    fields: Vec<Field>,

    /// Default text formatting stuff.
    text_settings: TextSettings,

    /// Header format, if any
    header: Option<Header>,

    /// Footer format, if any
    footer: Option<Footer>,

    /// Frontmatter, if any
    frontmatter: Option<Frontmatter>,

    /// Backmatter, if any
    backmatter: Option<Backmatter>,

    /// Bibliography, if any
    bibliography: Option<Bibliography>,

    /// Body formatter
    body: Body
}

impl Strategy {

    /// Create a Strategy from an expression
    ///
    /// At this stage, it is too late to preprocess.
    /// As such, we don't need a working directory.
    pub fn new(expr: syntax::Node) -> Result<Strategy> {
        Err(Error::Unimplemented(file!(), line!()))
    }

    /// Create a Strategy from a source file.
    pub fn new_from_file(file: &Path) -> Result<Strategy> {
        let reader = BufReader::new(try!(File::open(file)));
        let dir = file.parent().unwrap(); // Should be safe? File already found, should have a folder.
        Strategy::new_from_reader(reader, Some(dir))
    }

    /// Create a Strategy from a source string, and a working directory.
    ///
    /// The working directory is necessary because of possible preprocessor commands.
    pub fn new_from_string(string: &str, working_dir: Option<&Path>) -> Result<Strategy> {
        let reader = BufReader::new(string.as_bytes());
        Strategy::new_from_reader(reader, working_dir)
    }

    /// Create a Strategy from any buf reader and working directory.
    pub fn new_from_reader<R: BufRead>(reader: R, working_dir: Option<&Path>) -> Result<Strategy> {
        let source = preprocess::preprocess(reader, working_dir);
        println!("{}", source);
        let mut tokens = try!(Lexer::new_from_string(source));
        let ast = match parse::next_node(&mut tokens) {
            Err(e) => return Err(e),
            Ok(None) => return Err(Error::ParseFailure),
            Ok(Some(node)) => node,
        };
        Strategy::new(ast)
    }
}

/// A grouping of a name, a field-type, a possible default value, and option-ality
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Field;

/// Text settings
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct TextSettings;

/// Header settings
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Header;

/// Footer settings
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Footer;

/// Frontmatter (like a title page, page of contents, etc)
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Frontmatter;

/// Backmatter (like a glossery of terms, or index)
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Backmatter;

/// Bibliography settings
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Bibliography;

/// Body settings
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Body;
