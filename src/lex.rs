//! Lexer module
use error::{ Error, Result };
use token::Token;

use std::io::{ BufRead, BufReader };
use std::io::Cursor;
use std::iter;

/// Utility functions useful to lexing, which don't fall into a specific structure
pub mod util {
    use std::char;
    /// Attempts to parse a character from the head of a [u8].
    ///
    /// Returns the character parsed and the number of bytes it contains.
    pub fn compose_char(slice: &[u8]) -> Option<(char, usize)> {
        if slice.len() == 0 {
            return None;
        }

        const U8_PER_U32: usize = 4;
        for len in 1 .. U8_PER_U32 {
            let mut buf = slice[0] as u32;
            for index in 1 .. len {
                if slice.len() <= index { return None; }
                buf = (buf << 8) | (slice[index] as u32);
            }
            let buf = buf;
            if let Some(c) = char::from_u32(buf) { return Some((c, len as usize)); }
        }
        return None;
    }
}

/// State of the lexer
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum LexerState {
    /// Normal lexer state
    Normal,
    /// Lexer state inside of quoted string literals
    InsideQuote,
    /// Lexer state inside of comments
    InsideComment
}

/// Type used for lexing buffers
#[derive(Debug)]
pub struct Lexer<R: BufRead> {
    /// The internal buffer
    buffer: LexerBuf<R>,
}

impl <R> Lexer<R>
        where R: BufRead {
    /// Construct a lexer
    pub fn new(source: R) -> Result<Lexer<R>> {
        let lexbuf = try!(LexerBuf::new(source));
        let lexer = Lexer {
            buffer: lexbuf
        };
        Ok(lexer)
    }
}

impl Lexer<BufReader<Cursor<Vec<u8>>>> {
    /// Construct a lexer
    pub fn new_from_string(source: String) -> Result<Lexer<BufReader<Cursor<Vec<u8>>>>> {
        let buf = BufReader::new(Cursor::new(source.into_bytes()));
        Lexer::new(buf)
    }
}

impl <R> iter::Iterator for Lexer<R>
        where R: BufRead {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        None
    }
}

/// Internal buffer, which wraps a BufRead.
///
/// Provides special (better?) forward-reading capability than a regular BufRead
#[derive(Debug)]
pub struct LexerBuf<R: BufRead> {
    reader: R,
    buf: Vec<u8>,
    buf_from: usize,
}

impl <R> LexerBuf<R>
        where R: BufRead {
    // Construct a lexer buffer
    fn new(source: R) -> Result<LexerBuf<R>> {
        let lexbuf = LexerBuf {
            reader: source,
            buf: Vec::new(),
            buf_from: 0,
        };
        Ok(lexbuf)
    }

    /// Guarantee that the internal buffer has a certain amount of content
    fn guarantee_length(&mut self, guarantee: usize) -> Result<()> {
        if self.remaining_length() > guarantee {
            Ok(())
        } else {
            try!(self.fill_buf());
            if self.remaining_length() > guarantee {
                Ok(())
            } else {
                Err(Error::LexFailure)
            }
        }
    }

    fn remaining_length(&self) -> usize {
        self.buf.len() - self.buf_from
    }

    fn advance(&mut self, advance_by: usize) -> Result<()> {
        // If we don't have enough, try filling first.
        if self.remaining_length() < advance_by {
            try!(self.fill_buf());
        }
        // If we have enough, advance. Otherwise, return an error
        if self.remaining_length() >= advance_by {
            self.buf_from += advance_by;
            Ok(())
        } else {
            Err(Error::LexFailure)
        }
    }

    /// Read and append to the internal buffer
    fn fill_buf(&mut self) -> Result<()> {
        let len;
        {
            let tmp_buf = try!(self.reader.fill_buf());
            len = tmp_buf.len();
            self.buf.push_all(tmp_buf);
        }
        self.reader.consume(len);

        // If we've consumed a large enough piece of the vector, even after
        // pushing new contents, it's time to flush it.
        let consumed_proportion = (self.buf_from as f64) / (self.buf.len() as f64);
        if consumed_proportion > 0.67 {
            let new_buf = self.buf.split_off(self.buf_from);
            self.buf = new_buf;
            self.buf_from = 0;
        }

        // Indicate an error if 0 bytes could be read.
        match len {
            0 => Err(Error::LexFailure),
            _ => Ok(())
        }
    }

    /// Reads a char. Attempts to advance even if the read fails.
    pub fn get_char(&mut self) -> Result<char> {
        let c = self.peek_char();
        try!(self.advance(1));
        c
    }

    /// Looks at the next char
    pub fn peek_char(&mut self) -> Result<char> {
        self.guarantee_length(4).ok();
        if let Some((c, _)) = util::compose_char(&self.buf[self.buf_from .. ]) {
            Ok(c)
        } else {
            Err(Error::LexFailure)
        }
    }

    /// Peek a string of up to `len` characters.
    ///
    /// `peek_str` will happily return a string of less than `len` characters, but never more.
    pub fn peek_str(&mut self, len: usize) -> Result<String> {
        let mut retval = String::new();
        let mut offset = 0;
        for _ in 0 .. len {
            self.guarantee_length(4).ok();
            let true_offset = self.buf_from + offset;
            if let Some((c, bytes)) = util::compose_char(&self.buf[true_offset ..]) {
                offset += bytes;
                retval.push(c);
            } else {
                return Err(Error::LexFailure);
            }
        }
        Ok(retval)
    }
}

/*
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
*/
