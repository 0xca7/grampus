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
        contains random utility functions
        used by multiple modules
    
    Author: 0xca7
*/

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::prelude::*;

use crate::grammar_parser::PRODUCTION;

/// check if a file located at `filepath` exists
pub fn file_exists(filepath: &String) -> bool {
    Path::new(filepath).exists()
}

/// check the first line of a grammar file `filename` and
/// extract the start symbol. check if this start symbol 
/// matches with the one `expected`.
pub fn check_start_symbol(filepath: &String, expected: &String) -> bool {

    let file = match fs::File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("to open file {}", filepath),
    };

    let mut buffer = BufReader::new(file);
    let mut line = String::new();

    buffer.read_line(&mut line).expect("Unable to read line");

    line = match line.split(PRODUCTION).next() {
        Some(s) => s.to_string(),
        None => return false,
    };

    // remove any whitespaces from the symbol
    line.retain(|c| !c.is_whitespace());

    if line == *expected {
        return true;
    }
    false
}



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
