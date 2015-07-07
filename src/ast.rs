use lexer::TextSpan;
use lexer::Token;

#[derive(Debug)]
pub struct Ast {
    pub repr: Vec<Node>,
}

impl Ast {
    pub fn sanitize(&mut self) {
        for mut node in self.repr.iter_mut() {
            if let &mut Node::Expression(_, ref mut e) = node {
                e.sanitize();
            }
        }
    }
}

#[derive(Debug)]
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
        Node::Variable(span, txt.to_string())
    }

    pub fn txt(span: TextSpan, txt: Token) -> Node {
        Node::Text(span, txt.to_string())
    }

    pub fn prop(span: TextSpan, key: Token, value: Node) -> Node {
        Node::Property(span, key.to_string(), Box::new(value))
    }

}

#[derive(Debug)]
pub struct Expression {
    pub repr: Vec<Node>
}

impl Expression {
    fn sanitize(&mut self) {
        // Due to some lifetime issues, create a new vector for the expression and swap the two
        use std::mem;

        let mut iterable = Vec::new();
        mem::swap(&mut iterable, &mut self.repr);

        // The general algorithm is, for each element:
        //   If we're not dealing with text, just put it back in the expression
        //   If we're dealing with text, append it to a buffer and:
        //     If the next one is text, append to buffer and look further.
        //     If the next one isn't text, the buffer goes back on the expression
        let mut iter = iterable.into_iter().peekable();
        while let Some(node) = iter.next() {
            if let Node::Text(first, t) = node {
                let mut builder = t;
                let mut second = first;
                loop {
                    if let Some(&Node::Text(span, ref t2)) = iter.peek() {
                        builder = builder + " " + t2;
                        second = span;
                    } else {
                        self.repr.push(Node::Text(TextSpan::merge(first, second), builder));
                        break;
                    };
                    iter.next();
                }
            } else {
                self.repr.push(node);
            }
        }
    }
}