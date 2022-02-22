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
        libaries and modules used in the
        grampus project
    Author: 0xca7
*/

/// utility functions
pub mod util;

/// the corpus for fuzzing
pub mod corpus;

/// the fuzzing component
pub mod fuzzer;

/// fuzzing statistics
pub mod stats;

/// various mutations
pub mod mutation;

/// functions to parse a grammar from a file
pub mod grammar_parser;

/// reads a grammar from a file, represents
/// a grammar inside the program
pub mod grammar;

/// internal representation of a syntax tree to 
/// derive from a grammar
pub mod syntax_tree;

/// scheduler to determine what mutations to apply
pub mod scheduler;
