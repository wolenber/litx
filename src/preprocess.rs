//! Module containing the preprocessor.

use error::Error;
use error::Result;
use lex;
use parse;
use syntax::Node;

use std::io::BufRead;
use std::io::Read;
use std::fs::File;
use std::path::Path;

/// A preprocessor command
#[derive(Debug)]
pub struct Command {
    /// The command that should be ran
    pub name: String,
    /// The arguments, split into sections
    pub sections: Vec<Vec<Node>>,
}

impl Command {
    /// Construct a new Command from a syntax tree
    pub fn new(ast: Node) -> Result<Command> {
        let values = match ast {
            Node::Expression(values) => values,
            _ => return Err(Error::ParseFailure)
        };
        let mut value_iter = values.into_iter();
        let name = match value_iter.next() {
            Some(Node::Word(word)) => word,
            _ => return Err(Error::ParseFailure)
        };
        let mut sections = Vec::new();
        let mut section = Vec::new();
        for val in value_iter {
            match val {
                Node::Divider => {
                    if section.len() > 0 { sections.push(section); }
                    section = Vec::new();
                },
                a => section.push(a)
            }
        }
        if section.len() > 0 { sections.push(section); }
        let command = Command {
            name: name,
            sections: sections
        };
        Ok(command)
    }

    /// Evaluate a command
    pub fn eval(self, working_directory: Option<&Path>) -> Result<String> {
        match &*self.name {
            "include" => self.eval_include(working_directory),
            _ => Err(Error::EvaluationFailure)
        }
    }

    /// Evaluate the #[{include}] command
    pub fn eval_include(self, working_directory: Option<&Path>) -> Result<String> {
        let working_directory = match working_directory {
            Some(p) => p,
            None    => return Err(Error::EvaluationFailure)
        };
        let relative_path = if self.sections.len() == 1 && self.sections[0].len() == 1 {
            match self.sections[0][0] {
                Node::Word(ref s) => s,
                Node::Text(ref s) => s,
                _ => return Err(Error::EvaluationFailure)
            }
        } else {
            return Err(Error::EvaluationFailure);
        };
        let relative_path = Path::new(&relative_path);
        let include_path = working_directory.join(relative_path);
        let mut file = try!(File::open(include_path));
        let mut file_contents = String::new();
        try!(file.read_to_string(&mut file_contents));
        // HACK This is probably the least efficient way to do this
        Ok(String::from_str(file_contents.trim()))
    }
}

/// Reads through a document, excecuting any commands.
///
/// Commands take the form of `#[{name args || ::you get || it}]`.
/// A command must fit on a single line, but that line can have arbitrary length.
pub fn preprocess<R: BufRead>(reader: R, working_directory: Option<&Path>) -> String {
    let mut buffer = String::new();
    let mut line_no = 0;
    for line in reader.lines() {
        line_no += 1;
        // FIXME unwrap is bad juju
        let line = line.unwrap();
        if let Some(expr) = as_preprocessor_command(&line) {
            match evaluate_command(&expr, working_directory) {
                Ok(output) => buffer.push_str(&output),
                Err(e) => {
                    println!("WARNING: Error:  Preprocessor command failed at line {}.", line_no);
                    println!("         Detail: {}.", e);
                    println!("         Action: Using line as plaintext, not preprocessor command.");
                    buffer.push_str(&expr);
                }
            }
        } else {
            buffer.push_str(&line);
        }
        // After each line, remember to push a newline.
        buffer.push('\n');
    }
    buffer
}

/// Tries to convert a single line into a preprocessor command.
///
/// Returns Some(command) if it detects a preprocessor command, and None otherwise.
pub fn as_preprocessor_command(line: &str) -> Option<String> {
    let trim = line.trim();
    if trim.len() >= 5                           // If the line is long enough
            && &trim[..3] == "#[{"               // and starts with #[{
            && &trim[trim.len() - 2..] == "}]" { // and ends with }]
        // Remove the #, convert to String, and return
        let expr = &trim[1..];
        Some(String::from_str(expr))
    } else {
        None
    }
}

/// Evaluate a single preprocessor line, with crunch removed.
pub fn evaluate_command(source: &str, working_directory: Option<&Path>) -> Result<String> {
    let mut tokens = lex::from_str(source);
    let ast = match try!(parse::next_node(&mut tokens)) {
        Some(node) => node,
        None => return Err(Error::ParseFailure),
    };
    let command = try!(Command::new(ast));
    command.eval(working_directory)
}
