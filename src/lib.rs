#![allow(clippy::tabs_in_doc_comments)] // Makefile uses tabs
#![deny(missing_docs)]

//! A lossless parser for Makefiles
//!
//! Example:
//!
//! ```rust
//! use std::io::Read;
//! let contents = r#"PYTHON = python3
//!
//! .PHONY: all
//!
//! all: build
//!
//! build:
//! 	$(PYTHON) setup.py build
//! "#;
//! let makefile: makefile_lossless::Makefile = contents.parse().unwrap();
//!
//! assert_eq!(makefile.rules().count(), 3);
//! ```

mod lex;
mod parse;

pub use parse::{Error, Identifier, Include, Makefile, ParseError, Rule, VariableDefinition};

/// Represents all possible syntax elements in a Makefile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum SyntaxKind {
    /// Represents whitespace (spaces, non-indenting tabs)
    WHITESPACE,
    /// Represents a newline character
    NEWLINE,
    /// Represents an indentation (tab at the beginning of a line)
    INDENT,
    /// Represents a comment (starts with #)
    COMMENT,
    
    // Operators and punctuation
    /// Operators like =, :=, ::, etc.
    OPERATOR,
    /// The dollar sign $ used in variable references
    DOLLAR,
    /// Opening parenthesis (
    LPAREN,
    /// Closing parenthesis )
    RPAREN,
    /// Comma ,
    COMMA,
    /// Backslash character
    BACKSLASH,
    /// Special token for backslash followed by newline (line continuation)
    LINE_CONTINUATION, 
    
    // Identifiers
    /// Variable names, targets, prerequisites
    IDENTIFIER,
    /// Quoted strings
    QUOTE,
    /// Text content in recipe lines
    TEXT,
    
    // Error
    /// Represents a syntax error
    ERROR,
    
    // Composite nodes
    /// The root node of the syntax tree (entire file)
    ROOT,
    /// A variable definition
    VARIABLE,
    /// A rule definition
    RULE,
    /// An expression (like a variable value)
    EXPR,
    /// A variable reference (like $(VAR))
    VARIABLE_REF,
    /// An include directive
    INCLUDE,
    /// A conditional directive (like ifdef, ifndef, etc.)
    CONDITIONAL,
    /// Indented lines outside of rules
    INDENTED_BLOCK,
    
    // Virtual tokens. These tokens are never produced by lexer,
    // they are inserted by parser to represent higher-level constructs.
    /// A virtual token representing a tab character
    TAB,
    /// A recipe line in a rule
    RECIPE_LINE,
}

/// Convert our `SyntaxKind` into the rowan `SyntaxKind`.
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

