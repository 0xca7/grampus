/*
    Description:
        the corpus for fuzzing.
        this consists of multiple syntax trees
        which are derived from a grammar by a 
        `Grammar` instance (generator.rs)
    Author:
        0xca7
*/

use std::fs::File;
use std::collections::HashSet;
use std::io::{BufWriter, Write};

extern crate xshift;
use xshift::XorShift64;

use crate::util::generate_seed;
use crate::grammar::Grammar;
use crate::syntax_tree::TreeNode;

/// derive a random sentence from a grammar resulting in a 
/// derivation tree, function is recursive
fn derive(rand: &mut XorShift64, tree: &mut TreeNode, 
    g: &mut Grammar) {

    // if the value we are currently at is a non-terminal,  
    // we have to expand it
    if !g.is_terminal(&tree.value) {

        // keep track of non-terminals in derivation
        g.no_non_terminals += 1;
        
        // unwrap safe, as the value is in the grammar for certain
        // here, we get all possible productions from the grammar
        let derivation = g.productions.get(&tree.value).unwrap();
        let mut ridx = 0;

        // if we haven't reached the max. non-terminals, 
        // we can expand at random
        if g.no_non_terminals < g.max_non_terminals {
            // generate a random number [0; num_productions]
            ridx = rand.rand() as usize % derivation.len();
        } else {
            // if we have reached the max, we choose the least-cost
            // expansion
            let mut max_item = usize::MIN;
            let mut min_item = usize::MAX;

            // here we get the min and max length of possible expansions
            for (i, item) in derivation.iter().enumerate() {
                if item.len() < min_item {
                    min_item = item.len();
                    ridx = i
                }
                if item.len() > max_item {
                    max_item = item.len()
                }
            }

            // if all expansions have the same length, 
            // choose a random one
            if max_item == min_item {
                ridx = rand.rand() as usize % derivation.len();
            }

        }

        // derive the child node 
        for item in &derivation[ridx] {
            tree.insert_child(item);
        }
            
        // now, derive further (unwrap safe, tree has children)
        for child in tree.children.as_mut().unwrap() {
            derive(rand, child, g);
        }

    } // if it is a non-terminal

}

/// the corpus for fuzzing
#[derive(Clone)]
pub struct Corpus {
    /// number of trees in forest
    forest_size:    usize,
    /// the syntax trees 
    forest:         Vec<TreeNode>,
    /// random number generator
    prng:           XorShift64,
    /// the `Grammar` used to generate syntax trees from
    grammar:        Grammar,
    /// start symbol of the grammar
    start_symbol:   String,
    /// the inputs for fuzzing, derived from syntax trees
    inputs:         Vec<String>,
}

impl Corpus {
    
    /// create a corpus instance 
    pub fn new(grammar_file: &String, start_symbol: &String,
        max_expansion: usize, forest_size: usize) -> Corpus {

        let prng = match XorShift64::new(generate_seed()) {
            Some(rng)   => rng,
            None        => panic!("invalid PRNG seed\n"),
        };

        let g = match Grammar::new(grammar_file, max_expansion) {
            Some(grammar) => grammar,
            None            => panic!("could not init grammar\n"),
        };

        let forest = Vec::with_capacity(forest_size);

        Corpus {
            forest_size:    forest_size,
            forest:         forest,
            prng:           prng,
            grammar:        g,
            start_symbol:   start_symbol.clone(),
            inputs:         Vec::new(),
        }
    
    } // pub fn new

    /// generate the corpus
    pub fn generate(&mut self) {

        // hashes of inputs
        let mut hashes = HashSet::new();

        // clear inputs
        self.forest.clear();
        self.inputs.clear();

        // generate a syntax tree for each tree in `forest`
        // and make sure there are no duplicates
        for _ in 0..self.forest_size {
    
            loop {
                let mut tree = TreeNode::new(&self.start_symbol);
                derive(&mut self.prng, &mut tree, &mut self.grammar);
                self.grammar.no_non_terminals = 0;

                // don't generate duplicates 
                let hash = tree.hash();
                // check if the list of hashes already 
                // contains the generated input
                if !hashes.contains(&hash) {
                    let mut input = String::new();
                    hashes.insert(hash);
                    tree.build(&mut input);
                    self.forest.push(tree);
                    self.inputs.push(input);
                    break;
                }
            } // loop

        } // for

    } // pub fn generate

    /// get a random input for fuzzing
    pub fn get_input(&mut self) -> String {
        // mut self because prng must be mutable
        self.inputs[self.prng.rand() as usize % self.inputs.len()]
            .clone()
    }

    /// write the corpus to a file
    pub fn write_corpus(&self) -> std::io::Result<()> {

        let filename = format!("corpus/corpus.txt");
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        for input in &self.inputs {
            write!(&mut writer, "{}\n", input)?;
        }

        Ok(())
    }

}


