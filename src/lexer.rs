//! Lexer module

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Token type
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Token {
    /// ''string''
    Quote(String),
    /// // comment
    Comment(String),
    /// string
    Word(String),
    /// ::string
    Key(String),
    /// $$string
    Var(String),
    /// [{
    Open,
    /// }]
    Close,
    /// ||
    Divider,
    /// \n\n
    BlankLine,
    /// Clutter whitespace
    Whitespace
}

impl Token {
    pub fn contents(&self) -> Option<&str> {
        let s = match *self {
            Token::Quote(ref s) => s,
            Token::Comment(ref s) => s,
            Token::Word(ref s) => s,
            Token::Key(ref s) => s,
            Token::Var(ref s) => s,
            _ => return None
        };
        Some(s)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Token::Quote(ref s) => write!(f, "''{}''", s),
            Token::Comment(ref s) => write!(f, "// {}", s),
            Token::Word(ref s) => write!(f, "{}", s),
            Token::Key(ref s) => write!(f, "::{}", s),
            Token::Var(ref s) => write!(f, "$${}", s),
            Token::Open => write!(f, "[{{"),
            Token::Close => write!(f, "}}]"),
            Token::Divider => write!(f, "||"),
            Token::BlankLine => write!(f, "(empty line)"),
            Token::Whitespace => write!(f, "(whitespace)"),
        }
    }
}

/// The Lexer
#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    remaining: &'a str,
}

impl <'a> Lexer <'a> {
    /// Create a new Lexer
    pub fn new(src: &'a str) -> Self {
        Lexer {
            source: src,
            remaining: src,
        }
    }
}

impl <'a> Iterator for Lexer<'a>  {
    type Item = (Token, TextSpan);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(token) = take_token(&mut self.remaining) {
                if let (Token::Whitespace, _) = token {
                    continue;
                } else {
                    let (token, span) = token;
                    let text_span = TextSpan::from(self.source, self.remaining, span);
                    return Some((token, text_span));
                }
            } else {
                return None;
            }
        }
    }
}

lexer! {
    fn take_token(tok: 'a) -> (Token, &'a str);

    r#"''[^('')]*''"# => (Token::Quote(tok[2 .. tok.len()-2].to_owned()), tok),
    r#"//(~(.*\n.*))"# => (Token::Comment(tok[2 ..].trim().to_owned()), tok),
    r#"[\r\n]+([ \t]*[\r\n])+"# => (Token::BlankLine, tok),
    r#"\[\{"# => (Token::Open, tok),
    r#"\}\]"# => (Token::Close, tok),
    r#"\|\|"# => (Token::Divider, tok),
    r#"::[^ \t\r\n]+"# => (Token::Key(tok[2 ..].to_owned()), tok),
    r#"$$[^ \t\r\n]+"# => (Token::Var(tok[2 ..].to_owned()), tok),

    // Word should be after all other non-whitespace tokens.
    // The messy blob of slashes and braces guarantees no interaction with [{, }], ||
    // This isn't necessary for the other control sequences, as they capture text.
    r#"[^(\[\{)(\]\})(\|\|) \t\r\n]+"# => (Token::Word(tok.to_owned()), tok),

    // Allowable newlines in the whitespace are spelled out explicitely.
    // This should guarantee that they don't interefere with blank lines.
    r#"[\r\n]+([ \t]*[\r\n]+)+"# => (Token::BlankLine, tok),
    r#"[ \t]+"# => (Token::Whitespace, tok),
    r#"[ \t]*\r\n"# => (Token::Whitespace, tok),
    r#"[ \t]*\n"# => (Token::Whitespace, tok),
}

/// A structure for grouping byte offset of text spans.
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub struct TextSpan {
    pub low: usize,
    pub high: usize,
}

impl TextSpan {
    /// Create a text span from a string and a slice of it.
    pub fn from(source: &str, remaining: &str, token: &str) -> TextSpan {
        let high = source.len() - remaining.len();
        let low = high - token.len();
        TextSpan { low: low, high: high }
    }

    pub fn merge(a: TextSpan, b: TextSpan) -> TextSpan {
        let low = if a.low < b.low { a.low } else { b.low };
        let high = if a.high > b.high { a.high } else { b.high };
        TextSpan { low: low, high: high }
    }
}


impl Display for TextSpan {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "[{}, {})", self.low, self.high)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that an input string lexes to a specific token
    fn test(input: &str, expected: Token) {
        let mut lexer = Lexer::new(input);
        let (actual, _) = lexer.next().unwrap();
        assert_eq!(expected, actual);
    }

    /// Test that the lex results of the next tokens match against a list of tokens
    fn test_tokens(input: &str, expected: &[Token]) {
        let mut lexer = Lexer::new(input);
        for expected_token in expected {
            let (actual, _) = lexer.next().unwrap();
            assert_eq!(*expected_token, actual);
        }
    }

    #[test]
    fn all_tokens() {
        let src = "\
            [{ outer
                ::inner [{foo bar}]
                ::quote ''this''
                ||
                $$var

                // Comment
                baz quux
            }]";
        let expected = [
            Token::Open,
                Token::Word("outer".to_owned()),
                Token::Key("inner".to_owned()),
                    Token::Open,
                        Token::Word("foo".to_owned()),
                        Token::Word("bar".to_owned()),
                    Token::Close,
                Token::Key("quote".to_owned()),
                    Token::Quote("this".to_owned()),
                Token::Divider,
                Token::Var("var".to_owned()),
                Token::BlankLine,
                Token::Comment("Comment".to_owned()),
                Token::Word("baz".to_owned()),
                Token::Word("quux".to_owned()),
            Token::Close,
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn expression_simple() {
        let src = "[{expr ::key val}]";
        let expected = [
            Token::Open,
            Token::Word("expr".to_owned()),
            Token::Key("key".to_owned()),
            Token::Word("val".to_owned()),
            Token::Close,
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn quote_empty() {
        let src = "''''";
        let expected = Token::Quote("".to_owned());
        test(src, expected);
    }

    #[test]
    fn quote_nonempty() {
        let src = "''Foo bar baz''";
        let expected = Token::Quote("Foo bar baz".to_owned());
        test(src, expected);
    }

    #[test]
    fn quote_with_tokens() {
        let src = "''[{ ::Foo Bar }]''";
        let expected = Token::Quote("[{ ::Foo Bar }]".to_owned());
        test(src, expected);
    }

    #[test]
    fn quote_ends_correctly() {
        let src = "''Test'' Foo bar";
        let expected = Token::Quote("Test".to_owned());
        test(src, expected);
    }

    #[test]
    fn quote_multiple() {
        let src = "''Foo'' ''Bar''";
        let expected = [
            Token::Quote("Foo".to_owned()),
            Token::Quote("Bar".to_owned()),
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn comment_single_line() {
        let src = "// Foo bar\n";
        let expected = Token::Comment("Foo bar".to_owned());
        test(src, expected);
    }

    #[test]
    fn comment_empty() {
        let src = "//\n";
        let expected = Token::Comment("".to_owned());
        test(src, expected);
    }

    #[test]
    fn comment_multiple_lines() {
        let src = "// Foo bar\n// Baz quux\n";
        let expected = [
            Token::Comment("Foo bar".to_owned()),
            Token::Comment("Baz quux".to_owned()),
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn comment_ends_correctly() {
        let src = "// Foo bar\nBaz quux";
        let expected = Token::Comment("Foo bar".to_owned());
        test(src, expected);
    }

    #[test]
    fn open() {
        let src = "[{";
        let expected = Token::Open;
        test(src, expected);
    }

    #[test]
    fn close() {
        let src = "}]";
        let expected = Token::Close;
        test(src, expected);
    }

    #[test]
    fn divider() {
        let src = "||";
        let expected = Token::Divider;
        test(src, expected);
    }

    #[test]
    fn key_ends_correctly() {
        let src = "::Key Value";
        let expected = Token::Key("Key".to_owned());
        test(src, expected);
    }

    #[test]
    fn var_ends_correctly() {
        let src = "$$foo bar";
        let expected = Token::Var("foo".to_owned());
        test(src, expected);
    }

    #[test]
    fn word_ends_correctly() {
        let src = "foo bar";
        let expected = Token::Word("foo".to_owned());
        test(src, expected);
    }
}