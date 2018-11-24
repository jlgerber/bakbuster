#[derive(Fail, Debug,PartialEq,Eq, PartialOrd, Ord)]
pub enum BBError {
    /// Parsing error
    #[fail(display = "failed to parse: {}", _0)]
    ParseError(String),
    /// General Errors originating in the Reqwest module
    /// Error raised when a feature is not implemented yet
    #[fail(display = "NotImplemented")]
    NotImplemented,
    /// Failure to find look up the specified variable in the environment
    #[fail(display = "{} not found in environment", _0)]
    EnvVarError(String),
     /// path provided does not exist
    #[fail(display = "{} does not exist", _0)]
    NonExtantPath(String),

}