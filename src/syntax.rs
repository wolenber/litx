//! Abstract syntax tree.

/// Represents a raw node of an abstract syntax tree.
#[derive(Debug)]
pub enum Node {
    /// A root node, containing 0 or more branches.
    Expression(Vec<Node>),
    /// A node containing a text key and exactly one child.
    Property(String, Box<Node>),
    /// A node which represents a variable value
    Var(String),
    /// A node containing exactly one word
    Word(String),
    /// A node containing any number of words
    Text(String),
    /// A node representing a divider
    Divider,
    /// A node representing a totally blank line
    BlankLine
}

impl Node {
    /// Gets the title of an expression, or None
    pub fn identity(&self) -> Option<&str> {
        let children = match self {
            &Node::Expression(ref children) => children,
            _ => return None
        };
        if children.len() == 0 {
            None
        } else if let Node::Word(ref name) = children[0] {
            Some(name)
        } else if let Node::Text(ref s) = children[0] {
            panic!("Node::get_title is partially unimplemented, found Text({})", s);
        } else {
            None
        }
    }

    /// Gets the text of text-only node, or None
    pub fn as_text<'a>(&'a self) -> Option<&'a str> {
        match self {
            &Node::Word(ref w) => Some(w),
            &Node::Text(ref t) => Some(t),
            _ => None,
        }
    }

    /// Gets the property in an expression, or None
    pub fn property<'a>(&'a self, key: &str) -> Option<&'a Node> {
        let children = match self {
            &Node::Expression(ref children) => children,
            _ => return None
        };
        for child in children {
            if let &Node::Property(ref ch_key, ref ch_value) = child {
                if key == *ch_key {
                    return Some(&*ch_value);
                }
            }
        }
        None
    }

    /// Gets the section of an expression (0 indexed, increasing at each ||)
    pub fn section<'a>(&'a self, section: usize) -> Option<&'a [Node]> {
        let children = match self {
            &Node::Expression(ref children) => children,
            _ => return None
        };
        let mut start_index = 0;
        let mut current_section = 0;
        let mut iter = children.iter();

        // Walk children until finding the correct section.
        // Note start_index increments even when a divider is found.
        while current_section < section {
            if let Some(node) = iter.next() {
                if let &Node::Divider = node {
                    current_section += 1;
                }
                start_index += 1;
            } else {
                return None;
            }
        }

        let mut end_index = start_index;
        // Walk children until finding another divider.
        // Note end_index increments even when a divider is found.
        while current_section == section {
            if let Some(node) = iter.next() {
                if let &Node::Divider = node {
                    current_section += 1;
                }
                end_index += 1;
            } else {
                // If we run out of iterator, just go from start_index to the end
                return Some(&children[start_index .. ]);
            }
        }
        Some(&children[start_index .. end_index])
    }
}
