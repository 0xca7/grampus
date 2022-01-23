/*
    Description:
        the main calls the fuzzer routine
        and prints a nice banner.

    Author: 0xca7
*/

use std::process;
use std::time::Instant;

use colored::*;
use clap::{Arg, App};

use grampus::util::{file_exists, check_start_symbol};
use grampus::fuzzer::fuzz;
use grampus::corpus::Corpus;

/// max width of expansions
const MAX_EXPANSION: usize = 3000;
/// number of inputs for fuzzing
const FOREST_SIZE:   usize = 1000;

/// fuzzer mode fuzz, does fuzzing on a target
const MODE_FUZZ:     &'static str = "fuzz";
/// this mode only generates a corpus
const MODE_GEN:      &'static str = "gen";

/// fancy banner, because ASCII art is cool
fn banner() {

print!("{}", r#"
       ____ __________ _____ ___  ____  __  _______
      / __ `/ ___/ __ `/ __ `__ \/ __ \/ / / / ___/
     / /_/ / /  / /_/ / / / / / / /_/ / /_/ (__  ) 
     \__, /_/   \__,_/_/ /_/ /_/ .___/\__,_/____/  
    /____/                    /_/                  

      - a bad grammar fuzzer by 0xca7

    MODES:
        'fuzz': fuzz mode fuzzes the supplied target.
                - requires the '-t' flag
        'gen' : only generates input for fuzzing, does
                no fuzz testing

    EXAMPLES:
        grampus -g grammars/ini.txt -s INI -m gen 
        -> generates fuzz inputs for fuzzing with a different,
           good fuzzer (for example AFL).
        grampus -g grammars/ini.txt -s INI -m fuzz -t target
        -> generates inputs from grammar ini.txt and uses them
           to fuzz the 'target'

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
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .value_name("mode")
            .help("the mode to run in, is either 'fuzz' or 'gen'")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("fuzz target")
            .short("t")
            .long("fuzz-target")
            .value_name("fuzz_target")
            .help("the program to fuzz")
            .required(false)
            .takes_value(true))
        .get_matches();

    // unwrap is safe, as all values are required.
    let grammar_file = matches.value_of("grammar file").unwrap().to_string();
    let start_symbol = matches.value_of("start symbol").unwrap().to_string();
    let mode = matches.value_of("mode").unwrap().to_string();
    
    // first, we need to know if the grammar file passed is valid
    if !file_exists(&grammar_file) {
        print!("grammar file does not exist\n");
        process::exit(1);
    }

    // ... and check the start symbol
    if !check_start_symbol(&grammar_file, &start_symbol) {
        print!("start symbol is not in grammar\n");
        process::exit(1);
    }

    // next is determining which mode we are in.
    // in fuzzing mode, we need a target
    if mode == "fuzz" {
        if matches.value_of("fuzz target").is_none() {
            print!("fuzz target not specified\n");
            process::exit(1);
        }
    }

    // regardless of the mode, we need a corpus to work with
    // so get a new corpus instance, this reads the grammar into
    // an internal representation
    let mut corpus = Corpus::new(
        &grammar_file, &start_symbol, MAX_EXPANSION, FOREST_SIZE
    );

    match &mode[..] {
        MODE_GEN => {
            print!("[+] generating a corpus\n");
            let now = Instant::now();
            corpus.generate();
            print!("[+] generation done, took {:?}.{:?} secs\n", now
                .elapsed().as_secs(), now.elapsed().as_millis());
            match corpus.write_corpus() {
                Ok(()) => print!("[+] wrote corpus\n"),
                Err(e) => print!("[!] error: {}", e),
            }
        },
        MODE_FUZZ => {
            let fuzz_target = matches.value_of("fuzz target").unwrap().to_string();
            if !file_exists(&fuzz_target) {
                print!("[!] fuzz target does not exist\n");
                process::exit(1);
            }
            // use the corpus for fuzzing a target
            print!("[+] starting fuzzer on target {}", fuzz_target);
            fuzz(corpus, &fuzz_target);
        },
        _         => print!("[!] error, unknown mode..."),
    }
       
}



