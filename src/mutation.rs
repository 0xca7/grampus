/*
    Description:
        mutations and mutation strategies
        for fuzzing.

    Author: 0xca7
*/

extern crate xshift;
use xshift::XorShift64;

use crate::util::generate_seed;

/// alias for mutation functions
pub type MutationFunction = fn(rand: &mut XorShift64, &Vec<u8>) -> Vec<u8>;

/// stores mutation types and applies a random mutation to an input
pub struct Mutator {
    /// to choose a random mutation to apply to an input
    prng: XorShift64,
    /// available mutations for random application
    mutations: Vec<MutationFunction>,
    /// maximum number of mutations
    max_mutations: usize,
}

impl Mutator {

    pub fn new(max_mutations: usize) -> Mutator {

        let prng = XorShift64::new(generate_seed()).unwrap();

        let remove:     MutationFunction = mutation_remove;
        let insert:     MutationFunction = mutation_insert;
        let bitflip:    MutationFunction = mutation_bitflip;
        let xor:        MutationFunction = mutation_xor;
        let arithmetic: MutationFunction = mutation_arithmetic;

        Mutator {
            prng:       prng,
            mutations:  vec![
                remove, insert, xor, arithmetic, bitflip
            ],
            max_mutations: max_mutations,
        }
    }

    /// take an input and apply `self.max_mutations` to it
    /// clones the `input` and returns the mutated result
    pub fn mutate(&mut self, input: &Vec<u8>) -> Vec<u8> {
    
        let mut mutation = input.clone();

        let n = (self.prng.rand() as usize) % self.max_mutations;
        
        for _ in 0..n {
            let f = self.mutations[(self.prng.rand() as usize) % 
                self.mutations.len()];
            mutation = f(&mut self.prng, &mutation);
    
            // don't remove too many characters
            if mutation.len() == 1 {
                break;
            }
    
        }
        mutation
    }

}

/// remove a byte from a vector
pub fn mutation_remove(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {
    let mut v = input.clone();
    if v.len() != 0 {
        v.remove((rand.rand() as usize) % v.len() );
    }
    v
}

/// insert a byte in a vector
pub fn mutation_insert(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {
    let mut v = input.clone();
    v.insert(
        (rand.rand() as usize) % v.len(),
        ((rand.rand() as usize) % 0xff) as u8
    );
    v
}

/// flip a single bit in a vector
pub fn mutation_bitflip(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

    let mut v = input.clone();

    // index in vector to flip
    let idx = (rand.rand() as usize) % v.len();
    
    // bit position to flip
    let pos = (rand.rand() as usize) % 8;
        
    // do flip
    let x = v[idx] ^ (1 << pos);

    // replace returns the old value, throw it away
    let _ = std::mem::replace(&mut v[idx], x);
    v
}

/// xor a single byte in a vector
pub fn mutation_xor(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

    let mut v = input.clone();

    // index in vector to flip
    let idx = (rand.rand() as usize) % v.len();
    
    // bit position to flip
    let r = ((rand.rand() as usize) % 0xff) as u8;
        
    // do xor
    let x = v[idx] ^ r;

    // replace returns the old value, throw it away
    let _ = std::mem::replace(&mut v[idx], x);
    v
}

/// takes a byte from an input and treats it as an integer
/// which arithmetic is applied to. here, it is a wrapping add.
pub fn mutation_arithmetic(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

    let mut v = input.clone();

    // index in vector to apply arithmetic to
    let idx = (rand.rand() as usize) % v.len();

    // wrapping add a random number
    // must be at least 1 and max is 255
    let x = v[idx].wrapping_add(((rand.rand() as usize) % 254 + 1) as u8);

    // replace returns the old value, throw it away
    let _ = std::mem::replace(&mut v[idx], x);
    v
}
