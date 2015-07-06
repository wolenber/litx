//! Token module

/// Token type
#[derive(Debug)]
pub enum Token {
    /// ''string''
    Quote(String),
    /// string
    Word(String),
    /// str in g
    TextLine(String),
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
    /// ::
    BareColon,
}
