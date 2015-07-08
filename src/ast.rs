use lexer::TextSpan;
use lexer::Token;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub struct Ast {
    pub repr: Vec<Node>,
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub enum Node {
    Expression(TextSpan, Expression),
    Divider(TextSpan),
    EmptyLines(TextSpan),
    Variable(TextSpan, String),
    Text(TextSpan, String),
    Property(TextSpan, String, Box<Node>),
}

impl Node {
    pub fn expr(span: TextSpan, repr: Vec<Node>) -> Node {
        Node::Expression(span, Expression {
            repr: repr
        })
    }

    pub fn div(span: TextSpan) -> Node {
        Node::Divider(span)
    }

    pub fn empty(span: TextSpan) -> Node {
        Node::EmptyLines(span)
    }

    pub fn var(span: TextSpan, txt: Token) -> Node {
        Node::Variable(span, txt.contents().unwrap().to_owned())
    }

    pub fn txt(span: TextSpan, txt: Token) -> Node {
        Node::Text(span, txt.contents().unwrap().to_owned())
    }

    pub fn prop(span: TextSpan, key: Token, value: Node) -> Node {
        Node::Property(span, key.contents().unwrap().to_owned(), Box::new(value))
    }
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub struct Expression {
    pub repr: Vec<Node>
}
