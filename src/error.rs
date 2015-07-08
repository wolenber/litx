//! Error type

use parser;

use std;
use std::io;
use std::fmt;
use std::fmt::{ Display };

/// Automatic impl for wrapping $from in $member
macro_rules! impl_from_error {
    (<$from:ty> for $to:ty as $member:path) => {
        impl From<$from> for $to {
            fn from(err: $from) -> $to {
                $member(err)
            }
        }
    }
}

/// Takes a comma seperated list of member paths.
///
/// This assumes that each error membor given is a tuple varient with a single field, the cause.
macro_rules! impl_error_cause {
    ( $($member:path),* ) => {
        fn cause(&self) -> Option<&std::error::Error> {
            match *self {
                $(
                    $member(ref cause) => Some(cause as &std::error::Error),
                )*
                _ => None
            }
        }
    }
}

/// Convenience newtype
pub type Result<T> = std::result::Result<T, Error>;

/// Litx standard error type
#[derive(Debug)]
pub enum Error {
    /// Error while lexing or parsing the document
    ParseFailure(parser::ParseError),
    /// Failure during evaluation and document building
    EvaluationFailure,
    /// Render failures
    RenderFailure,
    /// IO error
    Io(io::Error),
    /// Unimplemented failure. You should not see this, as a user
    Unimplemented(&'static str, u32),
}

impl_from_error!(<io::Error> for Error as Error::Io);
impl_from_error!(<parser::ParseError> for Error as Error::ParseFailure);

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseFailure(..) => "Indicates a failure occured during lexing or parsing",
            Error::EvaluationFailure => "Indicates a failure occured during an evaluation",
            Error::RenderFailure => "Indicates a failure occured during rendering",
            Error::Unimplemented(..) => "I haven't finished something yet. This isn't your fault.",
            Error::Io(_) => "Indicates an error occured during some io operations",
        }
    }

    impl_error_cause!(Error::Io);
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: These messages are trash.
        let msg = match *self {
            Error::ParseFailure(ref cause) => {
                match cause {
                    &(Some((ref token, span)), strin) =>
                        format!("Parsing Failure: {} @ {} ({})", token, span, strin),
                    &(None, strin) => format!("Parsing Failure: {}", strin),
                }
            }
            Error::EvaluationFailure => "Evaluation Failure".to_string(),
            Error::RenderFailure => "Render Failure".to_string(),
            Error::Unimplemented(file, line) => format!("Unimplemeted:  {}:{}", file, line),
            Error::Io(ref cause) => format!("IO Error:  {}", cause),
        };
        write!(fmt, "{}", msg)
    }
}
