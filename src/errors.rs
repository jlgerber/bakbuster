#[derive(Fail, Debug,PartialEq,Eq, PartialOrd, Ord)]
pub enum BBError {
    /// Parsing error
    #[fail(display = "ParseError: failed to parse: {}", _0)]
    ParseError(String),
    /// General Errors originating in the Reqwest module
    /// Error raised when a feature is not implemented yet
    #[fail(display = "NotImplemented")]
    NotImplemented,
    /// Failure to find look up the specified variable in the environment
    #[fail(display = "EnvVarError: {} not found in environment", _0)]
    EnvVarError(String),
     /// path provided does not exist
    #[fail(display = "NonExtantPath: {} does not exist", _0)]
    NonExtantPath(String),
    #[fail(display = "ConversionError: {}", _0)]
    ConversionError(String),


}