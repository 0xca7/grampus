/*
    Description:
        Input Generation from a given Grammar

    Author: 0xca7
*/
use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::grammar_parser::{parse_grammar};

/// contains a grammar and options for generation
#[derive(Debug)]
pub struct Grammar {

    /// the grammar represented via productions
    pub productions: HashMap<String, Vec<Vec<String>>>,
    /// the set of terminals in the grammar
    terminals: HashSet<String>,
    /// the set of non-terminals in the grammar
    non_terminals: HashSet<String>,

    /// number of non-terminals in tree
    pub no_non_terminals: usize,
    /// max. non-terminals in the tree at any time
    pub max_non_terminals: usize,
}

impl Grammar {

    /// parses a grammar file, if successful, creates
    /// a new instance of a `Generator`
    pub fn new(grammar_file: &String, 
        max_non_terminals: usize) -> Option<Grammar> {

        let mut productions: HashMap<String, Vec<Vec<String>>>
            = HashMap::new();
        let mut terminals = HashSet::new();
        let mut non_terminals = HashSet::new();

        match parse_grammar(grammar_file, &mut productions, 
            &mut terminals, &mut non_terminals) {
            Ok(_) => (),
            Err(e) => {
                print!("error parsing grammar: {}\n", e);
                return None;
            },
        };

        Some(Grammar {
            productions:        productions.clone(),
            terminals:          terminals.clone(),
            non_terminals:      non_terminals.clone(),
            no_non_terminals:   0,
            max_non_terminals:  max_non_terminals,
        })

    }

    /// check if a term is a terminal or not
    pub fn is_terminal(&self, term: &String) -> bool {
        self.terminals.contains(term)
    }

}

/// NOTE:   this is a little problematic, does not 
///         print in the right order (which is expected
///         when you have a hashmap), mostly there for debugging.
impl fmt::Display for Grammar {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (nterm, prod) in &self.productions {
            write!(f, "{} -> {:?}\n", nterm, prod)?;
        }
        write!(f, "\n")
    }

}
