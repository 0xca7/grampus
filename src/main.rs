//
// Takes a grammar G and generates sentences from it
// this implementation aims at acceptable performance
// and good readability. Additionally, in future implementations
// grammars are parsed in JSON format and used internally 
//
// 0xca7

use std::env;
use std::time::Instant;
use std::collections::HashSet;

extern crate xshift;
use xshift::XorShift64;

use grampus::generator::Generator;
use grampus::syntax_tree::TreeNode;

const MAX_EXPANSION: usize = 200;
const FOREST_SIZE:   usize = 100;

/**
 * TODO:
 *      - test perf
 *      - way to mutate, that is replace subtrees, is needed
 *        to get a fuzzer in the style of nautilus  
 *      - parse JSON internal representation of a grammar
 *      - as always, more comments
 */


/// derive a random sentence from a grammar resulting in a 
/// derivation tree, function is recursive
fn derive(rand: &mut XorShift64, tree: &mut TreeNode, 
    g: &mut Generator) {

    // if the value we are currently at is a non-terminal,  
    // we have to expand it
    if !g.is_terminal(&tree.value) {

        // keep track of non-terminals in derivation
        g.no_non_terminals += 1;
        
        // unwrap safe, as the value is in the grammar for certain
        // here, we get all possible productions from the grammar
        let derivation = g.grammar.get(&tree.value).unwrap();
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

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut forest = Vec::with_capacity(FOREST_SIZE);
    
    if args.len() < 3 {
        print!("Usage: grampus [Grammar File] [Start Symbol]\n");
        return;
    }
    
    let g = Generator::new(&args[1], MAX_EXPANSION);

    if g.is_none() {
        panic!("ERROR: could not initialize generator\n");
    } else {
    
        let mut g = g.unwrap();

        // this returns None when the seed is invalid
        // thus, do a check here.
        let mut rand = match XorShift64::new(
            unsafe { core::arch::x86_64::_rdtsc() } ) {
            Some(rng)   => rng,
            None        => panic!("invalid RNG seed, aborting\n"),
        };

        for _ in 0..FOREST_SIZE {
            forest.push(TreeNode::new(&args[2]));
        }

        let ts = Instant::now();

        for mut tree in &mut forest {
            derive(&mut rand, &mut tree, &mut g);
            g.no_non_terminals = 0;
        }

        let mut hashes = HashSet::new();

        for tree in &forest {
            let mut s = String::new();
            let h = tree.hash();
            if !hashes.contains(&h) {
                hashes.insert(h);
                tree.build(&mut s); 
                print!("{:#x}:  {}\n", h, s)
            }
        }

        print!("\n\ntime taken: {:?}\n", ts.elapsed());
    }
}
