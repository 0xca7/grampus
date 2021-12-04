//
// Xorshift library for reuse
// 0xca7
//

/// Xorshift64 for fast pseudo-random numbers
pub struct XorShift64 {
    /// the internal state
    x: u64,
}

impl XorShift64 {

    /// creates a new XorShift64 instance
    /// if seeded with `0` this returns `None` as 0 is an invalid seed
    /// else, `Some(...)` is returned
    pub fn new(seed: u64) -> Option<XorShift64> {
        // can't seed Xorshift with zero
        if seed == 0 {
            return None;
        }
        Some(XorShift64 { x: seed })
    }

    /// generates a new pseudo-random number
    /// returned as an unsigned 64-bit integer
    pub fn rand(&mut self) -> u64 {
       self.x ^= self.x << 13;
       self.x ^= self.x >> 7;
       self.x ^= self.x << 17;
       self.x
    }

}


// TESTING 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_xorshift64_invalid_new() {
        // test if an invalid seed results in a `None`
        let x = XorShift64::new(0);
        assert_eq!(x.is_some(), false);
    }

    #[test]
    fn test_xorshift64_valid_new() {
        // test if a valid seed results in a `Some`
        assert_eq!(XorShift64::new(0xdeadbeef).is_some(), true);
    }

    #[test]
    fn test_xorshift64_sanity() {
        // perform a sanity check
        // used C code from wikipedia to generate random sequence below
        let mut x = XorShift64::new(1234567).unwrap();
        assert_eq!(x.rand(), 1333627000697578);
        assert_eq!(x.rand(), 14261447996154253071);
        assert_eq!(x.rand(), 3585844736910023377);
        assert_eq!(x.rand(), 5442475313099128100);
        assert_eq!(x.rand(), 933003675054162526);
    }
}
