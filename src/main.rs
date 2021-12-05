/*
    Description:
        the main calls the fuzzer routine
        and prints a nice banner.

    Author: 0xca7
*/
use std::time::Instant;
use colored::*;

use grampus::fuzzer::fuzz;
use grampus::corpus::Corpus;

const MAX_EXPANSION: usize = 200;
const FOREST_SIZE:   usize = 100;

/// fancy banner, because ASCII art is cool
fn banner() {

print!("{}", r#"
       ____ __________ _____ ___  ____  __  _______
      / __ `/ ___/ __ `/ __ `__ \/ __ \/ / / / ___/
     / /_/ / /  / /_/ / / / / / / /_/ / /_/ (__  ) 
     \__, /_/   \__,_/_/ /_/ /_/ .___/\__,_/____/  
    /____/                    /_/                  

      - a bad grammar fuzzer by 0xca7
"#.yellow().bold());

}

fn main() {
    
    banner();

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
        
    fuzz(inputs);
    
}
