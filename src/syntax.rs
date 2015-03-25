//! Abstract syntax tree.

/// Represents a node of an abstract syntax tree.
#[derive(Debug)]
pub enum Node {
    /// A root node, containing 0 or more branches.
    Expression(Vec<Node>),
    /// A node containing a text key and exactly one child.
    Property(String, Box<Node>),
    /// A node containing exactly one word
    Word(String),
    /// A node containing any number of words
    Text(String),
    /// A node representing a divider
    Divider,
    /// A node representing a totally blank line
    BlankLine
}
