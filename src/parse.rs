//! Converting between a token stream and an AST is hard.
//! So we do it for you!

use error::Error;
use error::Result;
use syntax::Node;
use token::Token;

/// Construct one or more ASTs (multiple could be contained in one token stream)
pub fn from_tokens<T>(tokens: &mut T) -> Vec<Node>
        where T: Iterator<Item=Token> {
    // Pull nodes from the iterator, pushing successes onto the buffer.
    // An Ok(None) indicates the iterator finished happily.
    let mut node_buf = Vec::new();
    loop {
        let next = next_node(tokens);
        // FIXME Unclosed opens should crash, not be silently ignored like this...
        match next {
            Ok(Some(node)) => node_buf.push(node),
            Ok(None) => break,
            Err(_) => break,
        }
    }
    node_buf
}

/// Construct exactly one node, and all its children
///
/// An Err is an end of stream, while an Ok(None) is a close.
/// FIXME This isn't the correct behavior forever.
pub fn next_node<T>(tokens: &mut T) -> Result<Option<Node>>
        where T: Iterator<Item=Token> {
    if let Some(token) = tokens.next() {
        match token {
            Token::Open => {
                let children = from_tokens(tokens);
                let expression = Node::Expression(children);
                Ok(Some(expression))
            },
            Token::Close => Ok(None),
            Token::Divider => Ok(Some(Node::Divider)),
            Token::BlankLine => Ok(Some(Node::BlankLine)),
            Token::Quote(s) => Ok(Some(Node::Text(s))),
            Token::TextLine(s) => Ok(Some(Node::Text(s))),
            Token::Word(s) => Ok(Some(Node::Word(s))),
            Token::Var(s) => Ok(Some(Node::Var(s))),
            Token::Key(key) => {
                match next_node(tokens) {
                    Err(e) => Err(e),
                    Ok(None) => Err(Error::ParseFailure),
                    Ok(Some(value)) => Ok(Some(Node::Property(key, Box::new(value)))),
                }
            },
            Token::BareColon => Err(Error::ParseFailure),
        }
    } else {
        Err(Error::ParseFailure)
    }
}
