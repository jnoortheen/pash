use std::fmt;

use ruff_text_size::TextRange;

use crate::TokenKind;

/// Represents represent errors that occur during parsing and are
/// returned by the `parse_*` functions.
#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: TextRange,
}

impl std::ops::Deref for ParseError {
    type Target = ParseErrorType;

    fn deref(&self) -> &Self::Target {
        &self.error
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at byte range {:?}", &self.error, self.location)
    }
}

impl From<LexicalError> for ParseError {
    fn from(error: LexicalError) -> Self {
        ParseError {
            location: error.location(),
            error: ParseErrorType::Lexical(error.into_error()),
        }
    }
}

impl ParseError {
    pub fn error(self) -> ParseErrorType {
        self.error
    }
}

/// Represents the different types of errors that can occur during parsing of an f-string.
#[derive(Debug, Clone, PartialEq)]
pub enum FStringErrorType {
    /// Expected a right brace after an opened left brace.
    UnclosedLbrace,
    /// An invalid conversion flag was encountered.
    InvalidConversionFlag,
    /// A single right brace was encountered.
    SingleRbrace,
    /// Unterminated string.
    UnterminatedString,
    /// Unterminated triple-quoted string.
    UnterminatedTripleQuotedString,
    /// A lambda expression without parentheses was encountered.
    LambdaWithoutParentheses,
}

impl std::fmt::Display for FStringErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use FStringErrorType::{
            InvalidConversionFlag, LambdaWithoutParentheses, SingleRbrace, UnclosedLbrace,
            UnterminatedString, UnterminatedTripleQuotedString,
        };
        match self {
            UnclosedLbrace => write!(f, "expecting '}}'"),
            InvalidConversionFlag => write!(f, "invalid conversion character"),
            SingleRbrace => write!(f, "single '}}' is not allowed"),
            UnterminatedString => write!(f, "unterminated string"),
            UnterminatedTripleQuotedString => write!(f, "unterminated triple-quoted string"),
            LambdaWithoutParentheses => {
                write!(f, "lambda expressions are not allowed without parentheses")
            }
        }
    }
}

/// Represents the different types of errors that can occur during parsing.
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum ParseErrorType {
    /// An unexpected error occurred.
    #[error("{0}")]
    OtherError(String),

    /// An empty slice was found during parsing, e.g `data[]`.
    #[error("expected index or slice expression")]
    EmptySlice,
    /// An empty global names list was found during parsing.
    #[error("global statement must have at least one name")]
    EmptyGlobalNames,
    /// An empty nonlocal names list was found during parsing.
    #[error("nonlocal statement must have at least one name")]
    EmptyNonlocalNames,
    /// An empty delete targets list was found during parsing.
    #[error("delete statement must have at least one target")]
    EmptyDeleteTargets,
    /// An empty import names list was found during parsing.
    #[error("expected one or more symbol names after import")]
    EmptyImportNames,
    /// An empty type parameter list was found during parsing.
    #[error("type parameter list cannot be empty")]
    EmptyTypeParams,

    /// An unparenthesized named expression was found where it is not allowed.
    #[error("unparenthesized named expression cannot be used here")]
    UnparenthesizedNamedExpression,
    /// An unparenthesized tuple expression was found where it is not allowed.
    #[error("unparenthesized tuple expression cannot be used here")]
    UnparenthesizedTupleExpression,
    /// An unparenthesized generator expression was found where it is not allowed.
    #[error("unparenthesized generator expression cannot be used here")]
    UnparenthesizedGeneratorExpression,

    /// An invalid usage of a lambda expression was found.
    #[error("lambda expression cannot be used here")]
    InvalidLambdaExpressionUsage,
    /// An invalid usage of a yield expression was found.
    #[error("yield expression cannot be used here")]
    InvalidYieldExpressionUsage,
    /// An invalid usage of a starred expression was found.
    #[error("starred expression cannot be used here")]
    InvalidStarredExpressionUsage,
    /// A star pattern was found outside a sequence pattern.
    #[error("star pattern cannot be used here")]
    InvalidStarPatternUsage,

    /// A parameter was found after a vararg.
    #[error("parameter cannot follow var-keyword parameter")]
    ParamAfterVarKeywordParam,
    /// A non-default parameter follows a default parameter.
    #[error("parameter without a default cannot follow a parameter with a default")]
    NonDefaultParamAfterDefaultParam,
    /// A default value was found for a `*` or `**` parameter.
    #[error("parameter with '*' or '**' cannot have default value")]
    VarParameterWithDefault,

    /// A duplicate parameter was found in a function definition or lambda expression.
    #[error("duplicate parameter {0:?}")]
    DuplicateParameter(String),
    /// A keyword argument was repeated.
    #[error("duplicate keyword argument {0:?}")]
    DuplicateKeywordArgumentError(String),

    /// An invalid expression was found in the assignment target.
    #[error("invalid assignment target")]
    InvalidAssignmentTarget,
    /// An invalid expression was found in the named assignment target.
    #[error("invalid named assignment target")]
    InvalidNamedAssignmentTarget,
    /// An invalid expression was found in the annotated assignment target.
    #[error("invalid annotated assignment target")]
    InvalidAnnotatedAssignmentTarget,
    /// An invalid expression was found in the augmented assignment target.
    #[error("invalid augmented assignment target")]
    InvalidAugmentedAssignmentTarget,
    /// An invalid expression was found in the delete target.
    #[error("invalid delete target")]
    InvalidDeleteTarget,

    /// A positional argument was found after a keyword argument.
    #[error("positional argument cannot follow keyword argument")]
    PositionalAfterKeywordArgument,
    /// A positional argument was found after a keyword argument unpacking.
    #[error("positional argument cannot follow keyword argument unpacking")]
    PositionalAfterKeywordUnpacking,
    /// An iterable argument unpacking was found after keyword argument unpacking.
    #[error("iterable argument unpacking cannot follow keyword argument unpacking")]
    InvalidArgumentUnpackingOrder,
    /// An invalid usage of iterable unpacking in a comprehension was found.
    #[error("iterable unpacking cannot be used in a comprehension")]
    IterableUnpackingInComprehension,

    /// Multiple simple statements were found in the same line without a `;` separating them.
    #[error("simple statements must be separated by newlines or semicolons")]
    SimpleStatementsOnSameLine,
    /// A simple statement and a compound statement was found in the same line.
    #[error("compound statements are not allowed on the same line as simple statements")]
    SimpleAndCompoundStatementOnSameLine,

    /// Expected one or more keyword parameter after `*` separator.
    #[error("expected one or more keyword parameter after '*' separator")]
    ExpectedKeywordParam,
    /// Expected a real number for a complex literal pattern.
    #[error("expected a real number in complex literal pattern")]
    ExpectedRealNumber,
    /// Expected an imaginary number for a complex literal pattern.
    #[error("expected an imaginary number in complex literal pattern")]
    ExpectedImaginaryNumber,
    /// Expected an expression at the current parser location.
    #[error("expected an expression")]
    ExpectedExpression,
    /// The parser expected a specific token that was not found.
    #[error("expected {expected}, found {found}")]
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },

    /// An unexpected indentation was found during parsing.
    #[error("unexpected indentation")]
    UnexpectedIndentation,
    /// The statement being parsed cannot be `async`.
    #[error("expected 'def', 'with' or 'for' to follow 'async', found {0}")]
    UnexpectedTokenAfterAsync(TokenKind),
    /// Ipython escape command was found
    #[error("IPython escape commands are only allowed in `Mode::Ipython`")]
    UnexpectedIpythonEscapeCommand,
    /// An unexpected token was found at the end of an expression parsing
    #[error("unexpected token at the end of an expression")]
    UnexpectedExpressionToken,

    /// An f-string error containing the [`FStringErrorType`].
    #[error("f-string: {0}")]
    FStringError(FStringErrorType),
    /// Parser encountered an error during lexing.
    #[error("lexical error: {0}")]
    Lexical(LexicalErrorType),
}

/// Represents an error that occur during lexing and are
/// returned by the `parse_*` functions in the iterator in the
/// [lexer] implementation.
///
/// [lexer]: crate::lexer
#[derive(Debug, Clone, PartialEq)]
pub struct LexicalError {
    /// The type of error that occurred.
    error: LexicalErrorType,
    /// The location of the error.
    location: TextRange,
}

impl LexicalError {
    /// Creates a new `LexicalError` with the given error type and location.
    pub fn new(error: LexicalErrorType, location: TextRange) -> Self {
        Self { error, location }
    }

    pub fn error(&self) -> &LexicalErrorType {
        &self.error
    }

    pub fn into_error(self) -> LexicalErrorType {
        self.error
    }

    pub fn location(&self) -> TextRange {
        self.location
    }
}

impl std::ops::Deref for LexicalError {
    type Target = LexicalErrorType;

    fn deref(&self) -> &Self::Target {
        self.error()
    }
}

impl std::error::Error for LexicalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error())
    }
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} at byte offset {}",
            self.error(),
            u32::from(self.location().start())
        )
    }
}

/// Represents the different types of errors that can occur during lexing.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum LexicalErrorType {
    // TODO: Can probably be removed, the places it is used seem to be able
    // to use the `UnicodeError` variant instead.
    #[doc(hidden)]
    #[error("Got unexpected string")]
    StringError,
    /// A string literal without the closing quote.
    #[error("missing closing quote in string literal")]
    UnclosedStringError,
    /// Decoding of a unicode escape sequence in a string literal failed.
    #[error("Got unexpected unicode")]
    UnicodeError,
    /// Missing the `{` for unicode escape sequence.
    #[error("Missing `{{` in Unicode escape sequence")]
    MissingUnicodeLbrace,
    /// Missing the `}` for unicode escape sequence.
    #[error("Missing `}}` in Unicode escape sequence")]
    MissingUnicodeRbrace,
    /// The indentation is not consistent.
    #[error("unindent does not match any outer indentation level")]
    IndentationError,
    /// An unrecognized token was encountered.
    #[error("Got unexpected token {tok}")]
    UnrecognizedToken { tok: char },
    /// An f-string error containing the [`FStringErrorType`].
    #[error("f-string: {0}")]
    FStringError(FStringErrorType),
    /// Invalid character encountered in a byte literal.
    #[error("bytes can only contain ASCII literal characters")]
    InvalidByteLiteral,
    /// An unexpected character was encountered after a line continuation.
    #[error("Expected a newline after line continuation character")]
    LineContinuationError,
    /// An unexpected end of file was encountered.
    #[error("unexpected EOF while parsing")]
    Eof,
    /// An unexpected error occurred.
    #[error("{0}")]
    OtherError(Box<str>),
}

#[cfg(target_pointer_width = "64")]
mod sizes {
    use crate::error::{LexicalError, LexicalErrorType};
    use static_assertions::assert_eq_size;

    assert_eq_size!(LexicalErrorType, [u8; 24]);
    assert_eq_size!(LexicalError, [u8; 32]);
}
