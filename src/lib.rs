// 
// libaries and modules used in the
// cfg_grammar project
//

/// utility functions
pub mod util;

/// the corpus for fuzzing
pub mod corpus;

/// the fuzzing component
pub mod fuzzer;

/// various mutations
//pub mod mutation;

/// functions to parse a grammar from a file
pub mod grammar_parser;

/// reads a grammar from a file, represents
/// a grammar inside the program
pub mod grammar;

/// internal representation of a syntax tree to 
/// derive from a grammar
pub mod syntax_tree;


