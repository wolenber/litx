//! Module containing the parser

use ast::*;
use lexer::*;
use lexer::Token::*;

parser! {
    fn __parse__(Token, TextSpan);

    // Ignore spans
    (a, b) {
        TextSpan::merge(a, b)
    }

    ast: Ast {
        nodes[e] => Ast { repr: e }
    }

    nodes: Vec<Node> {
        => Vec::new(),
        nodes[mut accum] node[n] => {
            accum.push(n);
            accum
        },
        nodes[ns] Comment => { ns }
    }

    node: Node {
        BlankLine => Node::empty(span!()),
        Open nodes[v] Close => Node::expr(span!(), v),
        Divider => Node::div(span!()),
        Var[v] => Node::var(span!(), v),
        Key[k] node[v] => Node::prop(span!(), k, v),
        Word[w] => Node::txt(span!(), w),
        Quote[q] => Node::txt(span!(), q),
    }
}

/// Tuple of a Token and a Unit. Basically just a token, but cleans some type signatures.
pub type TokenSpan = (Token, TextSpan);

/// Parse an iterator of tokens into an AST.
pub fn parse<I: Iterator<Item=TokenSpan>>(i: I) 
        -> Result<Ast, (Option<TokenSpan>, &'static str)> {
    let mut ast = try!(__parse__(i));
    ast.sanitize();
    Ok(ast)
}