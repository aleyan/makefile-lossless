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
    WHITESPACE,
    NEWLINE,
    INDENT,
    COMMENT,
    
    // Operators and punctuation
    OPERATOR,
    DOLLAR,
    LPAREN,
    RPAREN,
    COMMA,
    BACKSLASH,
    LINE_CONTINUATION, 
    
    // Identifiers
    IDENTIFIER,
    QUOTE,
    TEXT,
    
    // Error
    ERROR,
    
    // Composite nodes
    ROOT,
    VARIABLE,
    RULE,
    EXPR,
    VARIABLE_REF,
    INCLUDE,
    CONDITIONAL,
    INDENTED_BLOCK,
    
    // Virtual tokens. These tokens are never produced by lexer,
    // they are inserted by parser to represent higher-level constructs.
    TAB,
    RECIPE_LINE,
}

/// Convert our `SyntaxKind` into the rowan `SyntaxKind`.
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

