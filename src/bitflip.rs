//
// bitflip l/s
// as implemented in AFL
//
// 0xca7
//

// pass this as an arg
use crate::xorshift::XorShift64;

#[derive(Debug, Clone, Copy)]
pub enum BitFlipCycle {
    Cycle1_1,
    Cycle2_1,
    Cycle4_1,
    Cycle8_8,
    Cycle16_8,
    Cycle32_8,
    CyclesDone,
}

/// a bitwalk is applied to a single, unique input
/// once `CyclesDone` is reached, the fuzzer can
/// continue with the next item in the cycle.
/// bitwalk consists of different strategies, applied
/// after one another.
#[derive(Debug)]
pub struct BitWalk {
    cycle: BitFlipCycle,
    idx: usize,
}

impl BitWalk {

    pub fn new() -> BitWalk {
        BitWalk { 
            cycle: BitFlipCycle::Cycle1_1, 
            idx: 0, 
        }
    }

    pub fn reset(&mut self) {
        self.cycle = BitFlipCycle::Cycle1_1;
        self.idx = 0;
    }
    
    /// return a string matching the current state
    pub fn show_state(&mut self) -> String {

        // if we are at the end of the input
        // go to the next cycle
        match self.cycle {
            BitFlipCycle::Cycle1_1 => "Bitflip 1/1".to_string(),
            BitFlipCycle::Cycle2_1 => "Bitflip 2/1".to_string(),
            BitFlipCycle::Cycle4_1 => "Bitflip 4/1".to_string(),
            BitFlipCycle::Cycle8_8 => "Bitflip 8/8".to_string(),
            BitFlipCycle::Cycle16_8 => "Bitflip 16/8".to_string(),
            BitFlipCycle::Cycle32_8 => "Bitflip 32/8".to_string(),
            BitFlipCycle::CyclesDone=> "Bitflip Cycle Done".to_string(),
        }
    }


    /// get the next state in the bitflip cycle
    fn next_state(&mut self, input_size: usize) -> BitFlipCycle {

        // if we are at the end of the input
        // go to the next cycle
        if self.idx >= input_size {

            self.idx = 0;

            match self.cycle {
                BitFlipCycle::Cycle1_1 => return BitFlipCycle::Cycle2_1,
                BitFlipCycle::Cycle2_1 => return BitFlipCycle::Cycle4_1,
                BitFlipCycle::Cycle4_1 => return BitFlipCycle::Cycle8_8,
                BitFlipCycle::Cycle8_8 => return BitFlipCycle::Cycle16_8,
                BitFlipCycle::Cycle16_8 => return BitFlipCycle::Cycle32_8,
                BitFlipCycle::Cycle32_8 => return BitFlipCycle::CyclesDone,
                BitFlipCycle::CyclesDone => return BitFlipCycle::CyclesDone,
            };
        }
        self.cycle
    }

    /**
     * why not call bitflip_1_1 two times to get bitflip_2_1?
     * because we have to clone two times. for bitflip_4_1 we
     * would have to clone 4 times. this way, we clone once.
     */

    /// flip one bit in each input byte
    fn bitflip_1_1(&mut self, 
        rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

        let mut v = input.clone();
        // choose a random bit use rng
        let r = rand.rand_range(0u64, 8u64);
        // flip the chosen bit for the current index
        v[self.idx] ^= (1 << r);
        self.idx += 1;
        v
    }

    /// flip two bits in each input byte
    fn bitflip_2_1(&mut self, 
        rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

        let mut v = input.clone();
        // choose a random bit use rng
        let r0 = rand.rand_range(0u64, 8u64);
        let r1 = rand.rand_range(0u64, 8u64);
        // flip the chosen bit for the current index
        v[self.idx] ^= (1 << r0);
        v[self.idx] ^= (1 << r1);
        self.idx += 1;
        v
    }

    /// flip four bits in each input byte
    fn bitflip_4_1(&mut self, 
        rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

        let mut v = input.clone();
        // choose a random bit use rng
        let r0 = rand.rand_range(0u64, 8u64);
        let r1 = rand.rand_range(0u64, 8u64);
        let r2 = rand.rand_range(0u64, 8u64);
        let r3 = rand.rand_range(0u64, 8u64);
        // flip the chosen bit for the current index
        v[self.idx] ^= (1 << r0);
        v[self.idx] ^= (1 << r1);
        v[self.idx] ^= (1 << r2);
        v[self.idx] ^= (1 << r3);
        self.idx += 1;
        v
    }

    fn bitflip_8_8(&mut self, 
        rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

        let mut v = input.clone();

        // flip 8 random bits in an 8 byte block
        for i in 0..8 {
            // get a random value in [0 ; 63]
            let r = rand.rand_range(0u64,64u64);
            // a block has length 8, so vector index = random / vector length
            // now get the shiftwidth, which is always r % vector length
            // this sets v[0..7] to (1 << (0..7))
            v[(r / 8) as usize] ^= 1 << (r % 8);
        }
        self.idx += 8;
        v
    }

    fn bitflip_16_8(&mut self, 
        rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

        let mut v = input.clone();

        // flip 16 random bits in an 8 byte block
        for i in 0..16 {
            // get a random value in [0 ; 63]
            let r = rand.rand_range(0u64,64u64);
            // a block has length 8, so vector index = random / vector length
            // now get the shiftwidth, which is always r % vector length
            // this sets v[0..7] to (1 << (0..7))
            let idx = (self.idx + (r / 8) as usize) % input.len();
            v[idx] ^= 1 << (r % 8);
        }
        self.idx += 8;
        v
    }

    fn bitflip_32_8(&mut self, 
        rand: &mut XorShift64, input: &Vec<u8>) -> Vec<u8> {

        let mut v = input.clone();

        // flip 16 random bits in an 8 byte block
        for i in 0..32 {
            // get a random value in [0 ; 63]
            let r = rand.rand_range(0u64,64u64);
            // a block has length 8, so vector index = random / vector length
            // now get the shiftwidth, which is always r % vector length
            // this sets v[0..7] to (1 << (0..7))
            let idx = (self.idx + (r / 8) as usize) % input.len();
            v[idx] ^= 1 << (r % 8);
        }
        self.idx += 8;
        v
    }

    pub fn walk(&mut self, rand: &mut XorShift64, input: &Vec<u8>) -> Option<Vec<u8>> {

        let res = match self.cycle {
            BitFlipCycle::Cycle1_1 => Some(self.bitflip_1_1(rand, input)),
            BitFlipCycle::Cycle2_1 => Some(self.bitflip_2_1(rand, input)),
            BitFlipCycle::Cycle4_1 => Some(self.bitflip_4_1(rand, input)),
            BitFlipCycle::Cycle8_8 => {
                // check input size if it is to small,
                // we can just abort here by returning `None`
                if input.len() < 8 {
                    return None;
                }
                Some(self.bitflip_8_8(rand, input))
            },
            BitFlipCycle::Cycle16_8 => {
                // check input size if it is to small,
                // we can just abort here by returning `None`
                if input.len() < 8 {
                    return None;
                }
                Some(self.bitflip_16_8(rand, input))
            },
            BitFlipCycle::Cycle32_8 => {
                // check input size if it is to small,
                // we can just abort here by returning `None`
                if input.len() < 8 {
                    return None;
                }
                Some(self.bitflip_32_8(rand, input))
            },
            _ => None,

        };

        self.cycle = self.next_state(input.len());
        res
    }

}
