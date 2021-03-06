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

        this module allows the parsing of a grammar from a file.
        for how the file specifiying the grammar must be structured,
        refer the the README.md.

    Notes:
        
        some functions in this file are not programmed in an optimal
        way. This is because I didn't want to use any crates or rust
        nightly. I hope I will find better ways to code these.

    Author: 0xca7
*/

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

// identify non-terminals
// '(.*?)'
use regex::Regex;

const WHITESPACE: &str = "^";

/// the symbol for replacement in a grammar.
/// example: S = aSb (= is replacement), S -> aabS (-> is replacement)
pub const PRODUCTION: &str = "::=";

/// delimiter symbol for productions
/// example: S = aSb | ab
const DELIMITER:  &str = "|";

/// split a production by whitespaces and apply the 
/// `split_non_terminals` function to it
pub fn split_production(text: &String) -> Vec<String> {

    // split by whitespace
    let temp = text.split(" ")
        .map(|x| x.to_string().replace(WHITESPACE," "))
        .collect::<Vec<String>>();

    let mut split = Vec::new();

    for item in temp {
        let mut s = split_non_terminals(&item);
        split.append(&mut s);
    }

    split
}

/// badly programmed function to split a non-terminal
/// and a terminal when they are written without a space
pub fn split_non_terminals(text: &String) -> Vec<String> {

    let mut temp = String::new();
    let mut result = Vec::new();
    let mut finish = Vec::new();
    let mut last = 0;
    
    let mut idx = 0;

    if text.len() < 3 {
        return vec![];
    }

    // split everything up
    for (index, matched) in text.
        match_indices( |c: char| (c == '\'') ) {

        if last != index {
            result.push(&text[last..index]);
        }

        result.push(matched);

        last = index + matched.len();
    }
    
    if last < text.len() {
        result.push(&text[last..]);
    }

    // recombine all '...'
    // this sucks...
    while idx < result.len() {

        if result[idx] == "'" {
            let res = &result[idx..idx+3];
            let mut temp = String::new();
            for item in res {
                temp.push_str(item);
            }
            finish.push(temp.clone());
            idx += 3;
        } else {
            temp = result[idx].to_string();
            finish.push(temp.clone());
            idx += 1;
        }

        temp.clear();
    }

    finish
}

/// remove whitespaces in beginning and ending of string
fn remove_begin_end_whitespace(s: &mut String) {
    let len: usize = s.len();
    if s.ends_with(" ") {
        s.truncate(len-1);
    } 
    if s.starts_with(" ") {
        let _ = s.remove(0);
    } 
}

/// strips the quotes from a string
fn strip_quotes(s: &mut String) {
    if s.len() >= 3 {
        *s = s[1..s.len()-1].to_string();
    }
}

/// removes the quotes from vector of strings containing terminals
fn remove_quotes(s: &mut Vec<String>) {

    // match everything between ''
    let re = Regex::new(r"'(.*?)'").unwrap();

    for item in s {
        if re.is_match(&*item) {
            strip_quotes(item);
        }
    }

}

/// check if `s` contains terminals, if yes, insert them into
/// the `terminals` set
fn extract_terminals(s: &Vec<String>, terminals: &mut HashSet<String>) {

    // match everything between ''
    let re = Regex::new(r"'(.*?)'").unwrap();

    for item in s {
        if re.is_match(&*item) {
            // add the terminal to the set of terminals, omitting quotes
            let mut terminal: String = item.clone();
            strip_quotes(&mut terminal);
            terminals.insert(terminal);
        }
    }

}

/// read a grammar file and parse it to a hashmap data structure
pub fn parse_grammar(file_name: &String, 
    grammar_productions: &mut HashMap<String, Vec<Vec<String>>>, 
    grammar_terminals: &mut HashSet<String>, 
    grammar_non_terminals: &mut HashSet<String>)
    -> std::io::Result<()> {

    // file containing grammar
    let file = File::open(file_name)?;

    // use a buffered reader to read the grammar file
    let mut buf_reader = BufReader::new(file);

    // file contents 
    let mut contents = String::new();

    // buffered read the contents of the file 
    buf_reader.read_to_string(&mut contents)?;

    // split into lines
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // each line is a production
    for line in lines {
    
        // #ca7# remove this, it sucks.
        if line == "" {
            break;
        }

        // get the production rule
        let rule = line.split(PRODUCTION).collect::<Vec<&str>>();

        // extract the non-terminals from the rule 
        // the LHS is always a non-terminal S -> aSb (S is non-terminal)
        let mut lhs = rule[0].to_string();
        remove_begin_end_whitespace(&mut lhs);
        grammar_non_terminals.insert(lhs.clone());

        // this is also the key
        let key = lhs;

        // assigned below
        let mut values: Vec<Vec<String>> = Vec::new();

        // now split up the second part of the rule to get a list of productions
        let productions = rule[1].split(DELIMITER).collect::<Vec<&str>>();

        // take apart all productions
        for production in productions {

            let mut item = production.to_string();
            // we don't need beginning and end whitespaces
            remove_begin_end_whitespace(&mut item); 

            // assignment below, declared here because of scope
            let mut split: Vec<String>;

            split = split_production(&item);

            // get the terminals
            extract_terminals(&split, grammar_terminals);
            // remove the quotes
            remove_quotes(&mut split);

            values.push(split);

        } // for each production 
    
        // now we have a key and a value vector which we can push to
        // our hash map
        grammar_productions.insert(key, values.clone());

        values.clear();
    }

    Ok(())

}
