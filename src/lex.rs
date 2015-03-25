//! Lexer module

#![allow(missing_docs)]
#![allow(missing_debug_implementations)]

use token::Token;

use std::io::BufReader;

pub fn from_str(source: &str) -> Lexer<BufReader<&[u8]>> {
    let reader = BufReader::new(source.as_bytes());
    Lexer::new(reader)
}

/// Lexer type
rustlex! Lexer {
    // Some nice simple tokens
    let OPEN = "[{";
    let CLOSE = "}]";
    let DIV = "||";

    // A comment is a // until a newline
    let COMMENT = "//" [^'\n']* '\n';

    // A word is anything that is not:
    //     already a token
    //     a comment
    //     a double-colon
    //     part of a quote
    //     whitespace
    let WORD = (
        ('[' [^'{']) |
        ('}' [^']']) |
        ('|' [^'|']) |
        ('/' [^'/']) |
        (':' [^':']) |
        ('\'' [^'\'']) |
        [^'[' '}' '|' '/' ':' '\'' ' ' '\t' '\n' '\r'])+;

    // Some conveniences for handling whitespace
    let SPACE = [' ' '\t'];
    let BREAK = '\n' '\r'?;
    let EMPTY_LINE = BREAK SPACE* BREAK;

    // A property is a :: immediately followed by word
    let COLON = "::";
    let PROPERTY = COLON WORD;

    // A string is everything from a '' to the next ''.
    // To escape a double quote within a string, use ::''
    let STRING = "''" ("::''" | [^"''"])* "''";

    // Convenience for grouping multiple words on the same line.
    let TEXT_LINE = (WORD SPACE?)+ WORD;

    // These are in order of least precidence
    TEXT_LINE     => |lexer: &mut Lexer<R>| {
        let s = lexer.yystr();
        if &*s != s.trim() {
            println!("WARNING: Poorly formed document. Consider quoting for");
            println!("         any word ending in brackets, braces, slashes,");
            println!("         pipes, colons, or a single-quote.");
        }
        Some(Token::TextLine(String::from_str(s.trim())))
    }
    WORD          => |lexer: &mut Lexer<R>| {
        let s = lexer.yystr();
        if &*s != s.trim() {
            println!("WARNING: Poorly formed document. Consider quoting for");
            println!("         any word ending in brackets, braces, slashes,");
            println!("         pipes, colons, or a single-quotes.");
        }
        Some(Token::Word(String::from_str(s.trim())))
    }
    OPEN          => |_|
        Some(Token::Open)
    CLOSE         => |_|
        Some(Token::Close)
    DIV           => |_|
        Some(Token::Divider)
    COLON         => |_|
        Some(Token::BareColon)
    PROPERTY      => |lexer: &mut Lexer<R>| {
        let s = lexer.yystr();
        Some(Token::Key(String::from_str(&s[2 ..])))
    }
    STRING        => |lexer: &mut Lexer<R>| {
        let s = lexer.yystr();
        Some(Token::Quote(String::from_str(&s[2 .. s.len()-2])))
    }
    SPACE | BREAK => |_|
        None
    EMPTY_LINE    => |_|
        Some(Token::BlankLine)
    COMMENT       => |_|
        None
}
