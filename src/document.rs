//! Document strategy

use error::{ Result };
use expression::Expression;
use lexer::Lexer;
use parser;

/// A document is a cool beans kinda character
#[derive(Debug)]
pub struct Document {
    strategy: Strategy,
}

impl Document {
    /// Create a new document
    pub fn new(source: &str, strat: Strategy) -> Result<Document> {
        let lexer = Lexer::new(source);
        let ast = try!(parser::parse(lexer));
        let expr = Expression::from_ast(ast).unwrap();
        println!("{:#?}", expr);
        let doc = Document {
            strategy: strat,
        };
        Ok(doc)
    }
}

/// A document strategy is a template used for handling certain features of a document.
/// The strategy contains default formatting information, as well as meta-fields.
#[derive(Debug)]
pub struct Strategy {
    /// The name of the strategy.
    name: String,
    /// Meta fields of the strategy, such as author's name.
    /// Theoretically, these fields are very small changes, and don't effect formatting
    meta: Vec<Field>,
    /// Fields of the strategy, such as page size and works-cited
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
    /// Create a new strategy from a source string
    pub fn new(source: &str) -> Result<Strategy> {
        let lexer = Lexer::new(source);
        let ast = try!(parser::parse(lexer));
        let expr = Expression::from_ast(ast).unwrap();
        println!("{:#?}", expr);
        let s = Strategy {
            name: "".to_owned(),
            meta: Vec::new(),
            fields: Vec::new(),
            text_settings: TextSettings,
            header: None,
            footer: None,
            frontmatter: None,
            backmatter: None,
            bibliography: None,
            body: Body,
        };
        Ok(s)
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
