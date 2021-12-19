/*
    Description:
        the main calls the fuzzer routine
        and prints a nice banner.

    Author: 0xca7
*/

use std::process;

use colored::*;
use clap::{Arg, App};

use grampus::util::{file_exists, check_start_symbol};
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

    let matches = App::new("Grampus")
        .version("0.1")
        .author("0xca7")
        .about("the crappy grammar fuzzer")
        .arg(Arg::with_name("grammar file")
            .short("g")
            .long("grammar-file")
            .value_name("grammar_file")
            .help("a file containing a grammar")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("start symbol")
            .short("s")
            .long("start-symbol")
            .value_name("start_symbol")
            .help("the start symbol of the grammar")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("fuzz target")
            .short("t")
            .long("fuzz-target")
            .value_name("fuzz_target")
            .help("the program to fuzz")
            .required(true)
            .takes_value(true))
        .get_matches();

    // unwrap is safe, as all values are required.
    let grammar_file = matches.value_of("grammar file").unwrap().to_string();
    let start_symbol = matches.value_of("start symbol").unwrap().to_string();
    let fuzz_target = matches.value_of("fuzz target").unwrap().to_string();

    if !file_exists(&fuzz_target) {
        print!("fuzz target does not exist\n");
        process::exit(1);
    }
    
    if !file_exists(&grammar_file) {
        print!("grammar file does not exist\n");
        process::exit(1);
    }

    if !check_start_symbol(&grammar_file, &start_symbol) {
        print!("start symbol is not in grammar\n");
        process::exit(1);
    }

    // get a new corpus instance, this reads the grammar into
    // an internal representation
    let corpus = Corpus::new(
        &grammar_file, &start_symbol, MAX_EXPANSION, FOREST_SIZE
    );
       
    // use the corpus for fuzzing a target
    fuzz(corpus, &fuzz_target);
}



