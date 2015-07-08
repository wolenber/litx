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

/// Parse an iterator of tokens into an AST.
pub fn parse<I: Iterator<Item=TokenSpan>>(i: I) 
        -> Result<Ast, ParseError> {
    let ast = try!(__parse__(i));
    Ok(ast)
}

/// Tuple of a Token and a Unit. Basically just a token, but cleans some type signatures.
pub type TokenSpan = (Token, TextSpan);

/// Error type automatically chosen by plex
pub type ParseError = (Option<TokenSpan>, &'static str);

#[cfg(test)]
mod test {
    use ast::*;
    use lexer::TextSpan;

    fn test(source: &str, expected: Ast) {
        use super::parse;
        use lexer::*;

        let lexer = Lexer::new(source);
        let parser = parse(lexer).unwrap();
        assert_eq!(expected, parser);
    }

    #[test]
    fn nothing() {
        let src = "";
        let expected = Ast { repr: Vec::new()};
        test(src, expected);
    }

    #[test]
    fn something() {
        let src = "[{ foo }]";
        let expected = Ast {
            repr: vec![
                Node::Expression ( TextSpan { low: 0, high: 9 }, Expression { repr: vec![
                    Node::Text( TextSpan { low: 3, high: 6 }, "foo".to_string())
                ]})
            ]
        };
        test(src, expected);
    }
}
