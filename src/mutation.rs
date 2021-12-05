//
// mutations and mutation strategies
// for fuzzing.
//
// 0xca7
//


extern crate xshift;
// fast pseudo-random numbers
use xshift::XorShift64;
// bitflip walk
use crate::bitflip::BitWalk;

/// max number of mutations to apply
const MAX_NUMBER_MUTATIONS: u64 = 100;

/// alias for mutation functions
pub type MutationFunction = fn(rand: &mut XorShift64, &Vec<u8>) -> Vec<u8>;

/// mutation strategies, taken from AFL
pub enum MutationStrategy {
    /// apply a stack of mutations on an input
    /// add and delete portions of the input
    Havoc,

    /// only bit-flips, additions, substitutions
    Deterministic,

    /// walk through all the bits in the input
    /// corresponds to bitflip L/S in AFL
    BitflipLS,
}

pub struct MutationCycle {
    /// current iterations in cycle
    it: u32,
    /// iterations per cycle
    ipc: u32,
    /// current state
    state: MutationStrategy,
    /// the rng used in a cycle
    rng: XorShift64,
    /// mutators deterministic
    mutators_deterministic: Vec<MutationFunction>,
    /// mutators havoc
    mutators_havoc: Vec<MutationFunction>,
    /// bitwalk L/S state
    bitflip: BitWalk,
}

impl MutationCycle {

    pub fn new(prng_seed: u64, ipc: u32) -> MutationCycle {

        let remove:     MutationFunction = mutation_remove;
        let insert:     MutationFunction = mutation_insert;
        let bitflip:    MutationFunction = mutation_bitflip;
        let xor:        MutationFunction = mutation_xor;
        let arithmetic: MutationFunction = mutation_arithmetic;

        MutationCycle { 
            it: 0,
            ipc: ipc,
            state: MutationStrategy::Deterministic,
            rng: XorShift64::new(prng_seed), 
            mutators_deterministic: vec![
                bitflip, xor, arithmetic,
            ],
            mutators_havoc: vec![
                bitflip, xor, arithmetic,
                insert, remove
            ],
            bitflip: BitWalk::new(),
        } // MutationCycle
    }

    pub fn reset(&mut self) {
        self.state = MutationStrategy::Deterministic;
        self.it = 0;
        self.bitflip.reset();
    }

    /// returns true if a cycle is complete,
    /// else false and a vector
    pub fn mutate(&mut self, input: &Vec<u8>) -> Option<Vec<u8>> {

        match self.state {
            MutationStrategy::Deterministic => {
                // select a random mutation and apply it.
                let f = self.mutators_deterministic[self.rng
                    .rand_range(0, self.mutators_deterministic.len() as u64) as usize];
                self.it += 1;
                if self.it == self.ipc {
                    self.it = 0;
                    self.state = MutationStrategy::Havoc;
                }
                Some(f(&mut self.rng, input))
            },
            MutationStrategy::Havoc => {
                // number of mutations to apply
                let mut res = input.clone();
                let n = self.rng.rand_range(2, MAX_NUMBER_MUTATIONS) as usize;
                for _i in 0..n {
                    let f = self.mutators_havoc[self.rng.rand_range(0, 
                        self.mutators_havoc.len() as u64) as usize];
                    res = f(&mut self.rng, &res);
                }
                self.it += 1;
                if self.it == self.ipc {
                    self.it = 0;
                    self.state = MutationStrategy::BitflipLS;
                }
                Some(res)
            },
            MutationStrategy::BitflipLS => {
                let res = self.bitflip.walk(&mut self.rng, input);
                if res.is_none() {
                    self.bitflip.reset();
                }
                res
            }
        } // match
    } // fn mutate

}



/// remove a byte from a vector
pub fn mutation_remove(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {
    let mut v = input.clone();
    if v.len() != 0 {
        v.remove(rand.rand_range(0, v.len() as u64) as usize);
    }
    v
}

/// insert a byte in a vector
pub fn mutation_insert(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {
    let mut v = input.clone();
    v.insert(
        rand.rand_range(0, v.len() as u64) as usize,    // position    
        rand.rand_byte()                                // element
    );
    v
}

/// flip a single bit in a vector
pub fn mutation_bitflip(rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

    let mut v = input.clone();

    // index in vector to flip
    let idx = rand.rand_range(0, v.len() as u64) as usize;
    
    // bit position to flip
    let pos = rand.rand_range(0, 8 as u64) as usize;
        
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
    let idx = rand.rand_range(0, v.len() as u64) as usize;
    
    // bit position to flip
    let r = rand.rand_byte();
        
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
    let idx = rand.rand_range(0, v.len() as u64) as usize;

    // wrapping add a random number
    // must be at least 1 and max is 255
    let x = v[idx].wrapping_add(rand.rand_range(1u64, 256u64) as u8);

    // replace returns the old value, throw it away
    let _ = std::mem::replace(&mut v[idx], x);
    v
}
