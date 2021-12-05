/*
    Description:
        The fuzzer component of grampus.
        Contains the worker threads.

    Author: 0xca7
*/

use std::thread;
use std::sync::{Arc, Mutex};

extern crate xshift;
use xshift::XorShift64;
    
use crate::util::*;

const NUMBER_THREADS: usize = 1;

/// worker thread for a fuzzer. mutates an input from the
/// corpus and passes it to the PUT. The resulting return code
/// from the PUT is then assessed and a crashfile is written if
/// the PUT crashed given the current input.
fn worker(thread_id: u32, corpus: Arc<Vec<String>>) {

    print!("[ Worker Thread {} started ]\n", thread_id);

    let corpus_size = corpus.len();

    let mut prng = XorShift64::new(
        (generate_seed() + (thread_id as u64 ) + 1) as u64
    ).unwrap();

    loop {
        // worker loop 
        // select a random input from the corpus
        let fuzz_input = corpus
            .get(prng.rand() as usize % corpus_size).unwrap();
        print!("[{}] {}\n", thread_id, fuzz_input);
        break;
    }

}

pub fn fuzz(inputs: Vec<String>) {

    let mut handles = Vec::new();
    let corpus = Arc::new(inputs);
        
    for i in 0..NUMBER_THREADS {
        let corpus = Arc::clone(&corpus);
        let handle = thread::spawn(move || {
            worker(i as u32, corpus);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
