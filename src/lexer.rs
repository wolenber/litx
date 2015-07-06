//! Lexer module

/// Token type
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Token<'a> {
    /// ''string''
    Quote(&'a str),
    /// // comment
    Comment(&'a str),
    /// string
    Word(&'a str),
    /// ::string
    Key(&'a str),
    /// $$string
    Var(&'a str),
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

/// The Lexer
#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    remaining: &'a str,
}

impl <'a> Lexer<'a> {
    /// Create a new Lexer
    pub fn new(src: &'a str) -> Lexer<'a> {
        Lexer {
            source: src,
            remaining: src,
        }
    }
}

impl <'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        loop {
            if let Some(token) = take_token(&mut self.remaining) {
                if token == Token::Whitespace {
                    continue;
                } else {
                    return Some(token);
                }
            } else {
                return None;
            }
        }
    }
}

lexer! {
    fn take_token(tok: 'a) -> Token<'a>;

    r#"''(~(''))*''"# => Token::Quote(&tok[2 .. tok.len()-2]),
    r#"//(~(.*\n.*))"# => Token::Comment(tok[2 ..].trim()),
    r#"[\r\n]+([ \t]*[\r\n])+"# => Token::BlankLine,
    r#"\[\{"# => Token::Open,
    r#"\}\]"# => Token::Close,
    r#"\|\|"# => Token::Divider,
    r#"::[^ \t\r\n]+"# => Token::Key(&tok[2 ..]),
    r#"$$[^ \t\r\n]+"# => Token::Var(&tok[2 ..]),

    // Word should be after all other non-whitespace tokens.
    // The messy blob of slashes and braces guarantees no interaction with [{, }], ||
    // This isn't necessary for the other control sequences, as they capture text.
    r#"[^(\[\{)(\]\})(\|\|) \t\r\n]+"# => Token::Word(tok),

    // Allowable newlines in the whitespace are spelled out explicitely.
    // This should guarantee that they don't interefere with blank lines.
    r#"[\r\n]+([ \t]*[\r\n]+)+"# => Token::BlankLine,
    r#"[ \t]+"# => Token::Whitespace,
    r#"[ \t]*\r\n"# => Token::Whitespace,
    r#"[ \t]*\n"# => Token::Whitespace,
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that an input string lexes to a specific token
    fn test(input: &str, expected: Token) {
        let mut lexer = Lexer::new(input);
        let actual = lexer.next().unwrap();
        assert_eq!(expected, actual);
    }

    /// Test that the lex results of the next tokens match against a list of tokens
    fn test_tokens(input: &str, expected: &[Token]) {
        let mut lexer = Lexer::new(input);
        for expected_token in expected {
            let actual = lexer.next().unwrap();
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
                Token::Word("outer"),
                Token::Key("inner"),
                    Token::Open,
                        Token::Word("foo"),
                        Token::Word("bar"),
                    Token::Close,
                Token::Key("quote"),
                    Token::Quote("this"),
                Token::Divider,
                Token::Var("var"),
                Token::BlankLine,
                Token::Comment("Comment"),
                Token::Word("baz"),
                Token::Word("quux"),
            Token::Close,
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn expression_simple() {
        let src = "[{expr ::key val}]";
        let expected = [
            Token::Open,
            Token::Word("expr"),
            Token::Key("key"),
            Token::Word("val"),
            Token::Close,
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn quote_empty() {
        let src = "''''";
        let expected = Token::Quote("");
        test(src, expected);
    }

    #[test]
    fn quote_nonempty() {
        let src = "''Foo bar baz''";
        let expected = Token::Quote("Foo bar baz");
        test(src, expected);
    }

    #[test]
    fn quote_with_tokens() {
        let src = "''[{ ::Foo Bar }]''";
        let expected = Token::Quote("[{ ::Foo Bar }]");
        test(src, expected);
    }

    #[test]
    fn quote_ends_correctly() {
        let src = "''Test'' Foo bar";
        let expected = Token::Quote("Test");
        test(src, expected);
    }

    #[test]
    fn comment_single_line() {
        let src = "// Foo bar\n";
        let expected = Token::Comment("Foo bar");
        test(src, expected);
    }

    #[test]
    fn comment_empty() {
        let src = "//\n";
        let expected = Token::Comment("");
        test(src, expected);
    }

    #[test]
    fn comment_multiple_lines() {
        let src = "// Foo bar\n// Baz quux\n";
        let expected = [
            Token::Comment("Foo bar"),
            Token::Comment("Baz quux"),
        ];
        test_tokens(src, &expected);
    }

    #[test]
    fn comment_ends_correctly() {
        let src = "// Foo bar\nBaz quux";
        let expected = Token::Comment("Foo bar");
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
        let expected = Token::Key("Key");
        test(src, expected);
    }

    #[test]
    fn var_ends_correctly() {
        let src = "$$foo bar";
        let expected = Token::Var("foo");
        test(src, expected);
    }

    #[test]
    fn word_ends_correctly() {
        let src = "foo bar";
        let expected = Token::Word("foo");
        test(src, expected);
    }
}