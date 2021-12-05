//
// generation of inputs from a grammar
//
// 0xca7
//

/*

EXPR_GRAMMAR: Generator = {
    "<start>":
        ["<expr>"],

    "<expr>":
        ["<term> + <expr>", "<term> - <expr>", "<term>"],

    "<term>":
        ["<factor> * <term>", "<factor> / <term>", "<factor>"],

    "<factor>":
        ["+<factor>",
         "-<factor>",
         "(<expr>)",
         "<integer>.<integer>",
         "<integer>"],

    "<integer>":
        ["<digit><integer>", "<digit>"],

    "<digit>":
        ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
}

*/

use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::parse_grammar::{Grammar, parse_grammar};

/// represents a grammar, which is used to derive
/// sentences from 
#[derive(Debug)]
pub struct Generator {
    /// the grammar represented via productions
    pub grammar: HashMap<String, Vec<Vec<String>>>,
    /// the set of terminals in the grammar
    terminals: HashSet<String>,
    /// the set of non-terminals in the grammar
    non_terminals: HashSet<String>,
    /// number of non-terminals in tree
    pub no_non_terminals: usize,
    /// max. non-terminals in the tree at any time
    pub max_non_terminals: usize,
}

impl Generator {

    pub fn new(grammar_file: &String, 
        max_non_terminals: usize) -> Option<Generator> {

        let mut g = Grammar::new();

        match parse_grammar(grammar_file, &mut g) {
            Ok(_) => (),
            Err(e) => {
                print!("error parsing grammar: {}\n", e);
                return None;
            },
        };

        Some(Generator {
            grammar:            g.productions.clone(),
            terminals:          g.terminals.clone(),
            non_terminals:      g.non_terminals.clone(),
            no_non_terminals:   0,
            max_non_terminals:  max_non_terminals,
        })

    }

    /*
    /// creates a new grammar 
    /// uses the grammar above
    pub fn new(max_non_terminals: usize) -> Generator {

        let mut g = HashMap::new();
        let mut terminals = HashSet::new();

        //"<start>":
        //    ["<expr>"],
        g.insert("START".to_string(),
            vec![
                vec!["EXPR".to_string()]
            ],
        );

        //"<expr>":
        //    ["<term> + <expr>", "<term> - <expr>", "<term>"],
        g.insert("EXPR".to_string(),
            vec![
                vec![
                    "TERM".to_string(),
                    "+".to_string(),
                    "EXPR".to_string(),    
                    ],
                vec![
                    "TERM".to_string(),
                    "-".to_string(),
                    "EXPR".to_string(),    
                    ],
                vec![
                    "TERM".to_string(),
                    ],
            ],
        );

        // "<term>":
        // ["<factor> * <term>", "<factor> / <term>", "<factor>"], 
        g.insert("TERM".to_string(),
            vec![
                vec![
                    "FACTOR".to_string(),
                    "*".to_string(),
                    "TERM".to_string(),
                    ],
                vec![
                    "FACTOR".to_string(),
                    "/".to_string(),
                    "TERM".to_string(),
                    ],
                vec![
                    "FACTOR".to_string(),
                ],
            ]
        );

        // "<factor>":
        // ["+<factor>",
        // "-<factor>",
        // "(<expr>)",
        // "<integer>.<integer>",
        // "<integer>"],

        g.insert("FACTOR".to_string(),
            vec![
                vec![
                    "+".to_string(),
                    "FACTOR".to_string(),
                    ],
                vec![
                    "-".to_string(),
                    "FACTOR".to_string(),
                    ],
                vec![
                    "(".to_string(),
                    "EXPR".to_string(),
                    ")".to_string(),
                    ],
                vec![
                    "INTEGER".to_string(),
                    ".".to_string(),
                    "INTEGER".to_string(),
                    ],
                vec![
                    "INTEGER".to_string(),
                    ],
            ],
        );

        //"<integer>":
        //["<digit><integer>", "<digit>"], 
        g.insert("INTEGER".to_string(),
            vec![
                vec![
                    "DIGIT".to_string(),
                    "INTEGER".to_string(),
                ],
                vec![
                    "DIGIT".to_string(),
                ],
            ],
        );


        g.insert("DIGIT".to_string(),
            vec![
                vec!["0".to_string()], vec!["1".to_string()],
                vec!["2".to_string()], vec!["3".to_string()],
                vec!["4".to_string()], vec!["5".to_string()],
                vec!["6".to_string()], vec!["7".to_string()],
                vec!["8".to_string()], vec!["9".to_string()]
            ]
        );

        terminals.insert("+".to_string());
        terminals.insert("-".to_string());
        terminals.insert("*".to_string());
        terminals.insert("/".to_string());
        terminals.insert(".".to_string());
        terminals.insert("(".to_string());
        terminals.insert(")".to_string());
        terminals.insert("0".to_string());
        terminals.insert("1".to_string());
        terminals.insert("2".to_string());
        terminals.insert("3".to_string());
        terminals.insert("4".to_string());
        terminals.insert("5".to_string());
        terminals.insert("6".to_string());
        terminals.insert("7".to_string());
        terminals.insert("8".to_string());
        terminals.insert("9".to_string());

        Generator {
            grammar: g,
            terminals: terminals,
            no_non_terminals: 0,
            max_non_terminals: max_non_terminals,
        }

    }
    */

    /// check if a term is a terminal or not
    pub fn is_terminal(&self, term: &String) -> bool {
        self.terminals.contains(term)
    }

}

impl fmt::Display for Generator {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (nterm, prod) in &self.grammar {
            write!(f, "{} -> {:?}\n", nterm, prod)?;
        }
        write!(f, "\n")
    }

}
