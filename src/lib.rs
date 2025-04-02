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
    /// Whitespace token (spaces, tabs outside of recipe context)
    WHITESPACE,
    /// Newline token (line endings)
    NEWLINE,
    /// Indentation token (tab or spaces at beginning of line)
    INDENT,
    /// Comment token (lines starting with #)
    COMMENT,
    
    // Operators and punctuation
    /// Operator token (=, :=, +=, etc. for variables and : for rules)
    OPERATOR,
    /// Dollar sign token for variable references
    DOLLAR,
    /// Left parenthesis token
    LPAREN,
    /// Right parenthesis token
    RPAREN,
    /// Comma token
    COMMA,
    /// Backslash token
    BACKSLASH,
    /// Line continuation token (backslash followed by newline)
    LINE_CONTINUATION, 
    
    // Identifiers
    /// Identifier token (names, targets, prerequisites)
    IDENTIFIER,
    /// Quote token (single or double quotes)
    QUOTE,
    /// Raw text token (used in recipe lines)
    TEXT,
    
    // Error
    /// Error token for syntax errors
    ERROR,
    
    // Composite nodes
    /// Root node of the syntax tree representing the entire makefile
    ROOT,
    /// Variable definition node (name = value)
    VARIABLE,
    /// Rule node (target: prerequisites)
    RULE,
    /// Expression node (for variable values and other expressions)
    EXPR,
    /// Variable reference node (e.g. $(VARIABLE))
    VARIABLE_REF,
    /// Include directive node (include, -include, sinclude)
    INCLUDE,
    /// Conditional directive node (ifdef, ifndef, etc.)
    CONDITIONAL,
    /// Indented block node (blocks of indented text)
    INDENTED_BLOCK,
    
    // Virtual tokens. These tokens are never produced by lexer,
    // they are inserted by parser to represent higher-level constructs.
    /// Tab character inserted by parser
    TAB,
    /// Recipe line in a rule (indented commands)
    RECIPE_LINE,
}

/// Convert our `SyntaxKind` into the rowan `SyntaxKind`.
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

