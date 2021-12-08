/*
    Description:
        The fuzzer component of grampus.
        Contains the worker threads.

    Author: 0xca7
*/

use std::thread;
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::time::Instant;

extern crate xshift;
use xshift::XorShift64;

extern crate fnv;
use fnv::FnvHash;
    
use crate::util::*;
use crate::stats::Stats;
use crate::mutation::Mutator;

const NUMBER_THREADS: usize = 8;
const MAX_NUMBER_MUTATIONS : usize = 4;

/// worker thread for a fuzzer. mutates an input from the
/// corpus and passes it to the PUT. The resulting return code
/// from the PUT is then assessed and a crashfile is written if
/// the PUT crashed given the current input.
fn worker(thread_id: u32, corpus: Arc<Vec<String>>, target: String,
    stats: Arc<Mutex<Stats>>) {

    print!("[ Worker Thread {} started ]\n", thread_id);

    let corpus_size = corpus.len();

    let mut prng = XorShift64::new(
        (generate_seed() + (thread_id as u64 ) + 1) as u64
    ).unwrap();

    let mut mutator = Mutator::new(MAX_NUMBER_MUTATIONS);

    let mut fnv = FnvHash::new();

    let target = target.clone();
    
    loop {
        // select a random input from the corpus
        let item = corpus
            .get(prng.rand() as usize % corpus_size).unwrap();

        // mutate the input randomly
        let fuzz_input = mutator.mutate(&item.clone()
            .into_bytes().to_vec());

        // write the input file 
        let input_filename = format!("{}",
            write_input_file(&fuzz_input, thread_id).unwrap());

        // launch PUT and get result
        
        let output = Command::new(&target)
            .arg(input_filename)
            .output();

        let res = output.unwrap().status.code();

        match res {
            Some(code) => {
                if code != 0 {
                    //println!("code: {} => {}", code, fuzz_input);
                }
            },
            None => { 
                match write_crashfile(&fuzz_input, 
                    fnv.hash(&fuzz_input[..])) {
                    Ok(_)   => (),
                    Err(e)  => print!("thread {} \
                        couldn't write crashfile: {}\n
                        fuzz input: {:?}\n",
                        thread_id, e, fuzz_input),
                }
              
                let mut _stats = stats.lock().unwrap();
                _stats.inc_crashes();
            },
        };

        // write stats for fuzz case
        let mut _stats = stats.lock().unwrap();
        _stats.inc_fuzz_cases();
    }

}

/// this is the main fuzzer routine, it starts 
/// `NUMBER_THREADS` fuzzing threads
/// `inputs` is the corpus of valid strings used for 
/// mutations by the fuzzer
pub fn fuzz(inputs: Vec<String>, target: &String) {

    let mut seconds = 0;
    let mut handles = Vec::new();

    // we're only reading here, so no mutex
    let corpus = Arc::new(inputs);
    // we're writing here, so mutex
    let stats = Arc::new(Mutex::new(Stats::new()));
        
    for i in 0..NUMBER_THREADS {
        let corpus = Arc::clone(&corpus);
        let stats = Arc::clone(&stats);
        let target = target.clone();
        let handle = thread::spawn(move || {
            worker(i as u32, corpus, target, stats);
        });
        handles.push(handle);
    }

    let now = Instant::now();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        seconds += 1;
        let mut _stats = stats.lock().unwrap();
        _stats.show_stats(&seconds, &now.elapsed());
    }

    // NOTE: resolve this
    for handle in handles {
        handle.join().unwrap();
    }
}

