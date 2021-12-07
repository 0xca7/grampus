//
// FNV Hash Implementation
// 0xca7
//

/*
algorithm fnv-1a is
    hash := FNV_offset_basis

    for each byte_of_data to be hashed do
        hash := hash XOR byte_of_data
        hash := hash × FNV_prime

    return hash 
 */

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME:        u64 = 0x100000001b3;

pub struct FnvHash {
    offset: u64,
}

impl FnvHash {

    pub fn new() -> FnvHash {
        FnvHash { offset: FNV_OFFSET_BASIS }
    }

    /// the hashing algorithm
    pub fn hash(&mut self, data: &[u8]) -> u64 {
        self.offset = FNV_OFFSET_BASIS;
        let mut hash: u64 = self.offset;
        for byte in data.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        self.offset = hash;
        self.offset
    }

}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hash() {
        let mut h = FnvHash::new();
        assert_eq!(h.hash(b""), 0xcbf29ce484222325);

        let mut h = FnvHash::new();
        assert_eq!(h.hash(b"a"), 0xaf63dc4c8601ec8c);

        let mut h = FnvHash::new();
        assert_eq!(h.hash(b"b"), 0xaf63df4c8601f1a5);
    }
}

