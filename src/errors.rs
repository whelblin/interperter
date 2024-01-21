/// Error structure for the possible errors that can occurs
/// during the tokenizing process
#[derive(Debug)]
pub enum Error{
    //Tokenizer errors
    WrongToken,
    PatternError,
    MatchError,
    // Parser errors
    PeekOutOfBounds,
    UnexpectedToken,
    TestError,
    // Executor Errors
    IdentifierDoesNotExist,
    StackOut,
    FunctionParameterUnmatch,
    // Runner Errors
    FileNotGiven,

}