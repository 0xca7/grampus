//
// Takes a grammar G and generates sentences from it
// this implementation aims at acceptable performance
// and good readability. Additionally, in future implementations
// grammars are parsed in JSON format and used internally 
//
// 0xca7

use std::time::Instant;

use grampus::corpus::Corpus;

const MAX_EXPANSION: usize = 200;
const FOREST_SIZE:   usize = 100;

/**
 * TODO:
 *      - test perf
 *      - way to mutate, that is replace subtrees, is needed
 *        to get a fuzzer in the style of nautilus  
 *      - as always, more comments
 */

fn main() {

    let grammar_file = "grammars/json.txt".to_string();
    let start_symbol = "JSON".to_string();

    let mut corpus = Corpus::new(
        &grammar_file, &start_symbol,
        MAX_EXPANSION, FOREST_SIZE
    );

    let ts = Instant::now();
    corpus.generate();
    print!("\n\ntime taken: {:?}\n", ts.elapsed());

    let inputs = corpus.get_inputs();
        
    //for input in inputs {
    //    print!("{}\n", input);
    //}
    
}
