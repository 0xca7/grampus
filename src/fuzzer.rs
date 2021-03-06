/*
    grampus - a crappy grammar fuzzer
    Copyright (C) 2022  0xca7

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

/*
    Description:
        The fuzzer component of grampus.
        Contains the worker threads.

    Author: 0xca7
*/

use std::thread;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use std::os::unix::process::ExitStatusExt;

extern crate fnv;
use fnv::FnvHash;
    
use crate::util::*;
use crate::stats::Stats;
use crate::corpus::Corpus;
use crate::mutation::{Mutator, MutatorType};
use crate::scheduler::{Scheduler, FuzzingCycle};

/// signal numbers
const SIGILL:   i32 = 4;
const SIGABRT:  i32 = 6;
const SIGBUS:   i32 = 7;
const SIGSEGV:  i32 = 11;

/// number of fuzzer threads
const NUMBER_THREADS:       usize = 8;
/// max number of fuzz cases per cycle
const MAX_ITERATIONS_CYCLE: usize = 10000;
/// maximum number of mutations applied to one input
const MAX_NUMBER_MUTATIONS: usize = 4;

/// worker thread for a fuzzer. 
/// the worker generates inputs from a `corpus`, mutates a
/// corpus item and passes it to the PUT. The resulting return code
/// from the PUT is then assessed and a crashfile is written if
/// the PUT crashed given the current input.
fn worker(thread_id: u32, corpus: Corpus, target: String,
    stats: Arc<Mutex<Stats>>) {

    // the scheduler dictates in which sub-cycle the fuzzer is,
    // which determines which mutations are applied to an input
    // the fuzzer is in a cycle for `MAX_ITERATIONS_CYCLE`
    let mut scheduler = Scheduler::new(MAX_ITERATIONS_CYCLE);

    // the corpus used for generating inputs from a grammar 
    let mut corpus = corpus;

    // hash algorithm to give each crashfile a unique ID 
    let mut fnv = FnvHash::new();
        
    // the name of the target application
    let target = target.clone();

    print!("[ {}: thread started ]\n", thread_id);
    
    // start with a deterministic mutator
    let mut mutator = Mutator::new(MutatorType::Deterministic, 
        MAX_NUMBER_MUTATIONS);
    
    // outer loop.
    // here, inputs are generated via a `Corpus`
    loop {

        // we generate a corpus 
        corpus.generate();

        // start fuzzing with the corpus
        // label is mainly for documentation / readability 
        'fuzz_loop: loop {

            // check if the current fuzzing cycle is over
            let (next_cycle, cycle) = scheduler.next();

            // if it is, choose a new mutator or regenerate the corpus
            // if all cycles are complete
            if next_cycle {
                match cycle {
                    // regerate the corpus if we are done with all cycles
                    FuzzingCycle::CycleRegenerate       => {
                        let mut _stats = stats.lock().unwrap();
                        _stats.inc_cycles();
                        break 'fuzz_loop;
                    }
                    FuzzingCycle::CycleDeterministic    => {
                        mutator = Mutator::new(
                            MutatorType::Deterministic,
                            MAX_NUMBER_MUTATIONS);
                    },
                    FuzzingCycle::CycleNonDeterministic => {
                        mutator = Mutator::new(
                            MutatorType::NonDeterministic,
                            MAX_NUMBER_MUTATIONS);
                    },
                    FuzzingCycle::CycleBitWalk => {
                        mutator = Mutator::new(MutatorType::BitWalk,
                            MAX_NUMBER_MUTATIONS);
                    },
                } // match 
            } // if a cycle change occurs

            // select a random input from the corpus
            let item = corpus.get_input();

            // mutate the input randomly
            let fuzz_input = mutator.mutate(&item.clone()
                .into_bytes().to_vec());

            // write the input file 
            let input_filename = format!("{}",
                write_input_file(&fuzz_input, thread_id).unwrap());

            // launch PUT and get result
            let mut child = Command::new(&target)
                .arg(input_filename)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to start process");

            let res = child.wait();

            // check the result, if there was a crash, write 
            // a crash file
            match res {
                // check the exit code for a signal
                Ok(status) => {
                    // check what signal we got
                    match ExitStatusExt::signal(&status) {
                        Some(sig) => {
                        match sig {
                            SIGILL | SIGABRT | SIGBUS | SIGSEGV => {

                            match write_crashfile(&fuzz_input, 
                                fnv.hash(&fuzz_input[..])) {
                                Ok(_)   => (),
                                Err(e)  => print!("thread {} \
                                    couldn't write crashfile: {}\n
                                    fuzz input: {:?}\n",
                                    thread_id, e, fuzz_input),
                            }

                            // if a crash was found it should go into
                            // the stats
                            let mut _stats = stats.lock().unwrap();
                            _stats.inc_crashes();
                            },
                            _ => print!("exited with unkown signal\n"),
                            }
                        }, // got a signal   
                        // no signal was received
                        None => (),
                    }
                },

            Err(e) => print!("wait failed: {}\n", e),
            }
            // write stats for fuzz cases
            let mut _stats = stats.lock().unwrap();
            _stats.inc_fuzz_cases();

        } // inner loop

    } // outer loop, fuzz_cyles
}

/// this is the main fuzzer routine, it starts 
/// `NUMBER_THREADS` fuzzing threads, targeting
/// the `target` application
pub fn fuzz(corpus: Corpus, target: &String) {

    let mut seconds = 0;
    let mut handles = Vec::new();

    // we're writing here, so mutex
    let stats = Arc::new(Mutex::new(Stats::new()));
        
    // each thread gets it's own copy of the corpus
    // so each thread can generate inputs for itself
    for i in 0..NUMBER_THREADS {
        let stats = Arc::clone(&stats);
        let target = target.clone();
        // each thread receives their own `Corpus` to generate
        // inputs from
        let corpus = corpus.clone();
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

