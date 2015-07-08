use ast;

#[derive(Debug)]
pub struct Expression {
    title: Option<String>,
    sections: Vec<Section>,
}

impl Expression {
    pub fn from_ast(mut ast: ast::Ast) -> Option<Expression> {
        if let ast::Node::Expression(_, e) = ast.repr.swap_remove(0) {
            let e = Expression::from(e);
            Some(e)
        } else {
            None
        }
    }

    // FIXME: This method is ugly and poorly written. Can be cleaned up.
    pub fn from(ast: ast::Expression) -> Expression {
        let mut exp = Expression {
            title: None,
            sections: Vec::new(),
        };

        // If the first element is text, take it.
        let repr = ast.repr;
        let repr = if let Some(&ast::Node::Text(_, ref s)) = repr.first() {
            exp.title = Some(s.clone());
            &repr[1 ..]
        } else {
            &repr[..]
        };

        let mut section = Section::new();
        let mut text_buf: Option<String> = None;
        for node in repr {
            if text_buf.is_some() {
                if let &ast::Node::Text(_, ref s) = node {
                    text_buf = text_buf.map(|buf| buf + " " + s);
                } else {
                    section.add_node(Node::Atom(text_buf.unwrap()));
                    text_buf = None;
                }
            } else {
                match node {
                    &ast::Node::Divider(_) => {
                        exp.add_section(section);
                        section = Section::new();
                    },
                    &ast::Node::Text(_, ref s) => text_buf = Some(s.clone()),
                    a @ _ => section.add_node(Node::from_ast_node(a.clone())),
                }
            }
        }
        if text_buf.is_some() {
            section.add_node(Node::Atom(text_buf.unwrap()));
        }
        exp.add_section(section);
        exp
    }

    fn add_section(&mut self, s: Section) {
        self.sections.push(s);
    }
}

#[derive(Debug)]
pub struct Section {
    pub content: Vec<Node>,
}

impl Section {
    fn new() -> Section {
        Section {
            content: Vec::new(),
        }
    }

    fn add_node(&mut self, n: Node) {
        self.content.push(n);
    }
}

#[derive(Debug)]
pub enum Node {
    Expr(Expression),
    Prop(String, Box<Node>),
    Atom(String),
    Var(String),
    Blank,
}

impl Node {
    pub fn from_ast_node(node: ast::Node) -> Node {
        match node {
            ast::Node::Expression(_, e) => Node::Expr(Expression::from(e)),
            ast::Node::Divider(_) => panic!("FIXME: But also you can't do ::prop ||"),
            ast::Node::EmptyLines(_) => Node::Blank,
            ast::Node::Variable(_, s) => Node::Var(s),
            ast::Node::Text(_, s) => Node::Atom(s),
            ast::Node::Property(_, s, bn) => Node::Prop(s, Box::new(Node::from_ast_node(*bn))),
        }
    }
}
