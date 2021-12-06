/*
    Description:
        contains random utility functions
        used by multiple modules
    
    Author: 0xca7
*/
use std::fs::File;
use std::io::prelude::*;

/// generates a seed for a RNG 
/// WARNING: contains unsafe code
pub fn generate_seed() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// write a crashfile with the input `fuzz` 
/// to a file "crashes/hash(`fuzz`).txt"
pub fn write_crashfile(fuzz: &Vec<u8>, hash: u64) -> std::io::Result<()> {

    let filename = format!("crashes/{:x}.txt", hash);
    match File::create(filename) {
        Ok(ref mut f) => {
            f.write_all(fuzz)?;
        },
        Err(e) => print!("error writing crashfile: {}", e),
    };
    Ok(())
}

/// write an input file for fuzzing the file has the name
/// of the thread `thread_id` that created it.
/// the filename is returned so it can be used as a parameter
/// for the PUT.
pub fn write_input_file(content: &Vec<u8>, thread_id: u32) 
    -> std::io::Result<String> {
    let filename = format!("fuzz_inputs/{:02x}.txt", thread_id);
    
    match File::create(&filename) {
        Ok(ref mut f) => {
            f.write_all(content)?;
        },
        Err(e) => print!("error writing input file: {}", e),
    };
    Ok(filename)
}
