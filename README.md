# Grampus
a crappy grammar fuzzer

![Grampus](https://github.com/0xca7/grampus/tree/main/doc/grampus.png)

# UNDER CONSTRUCTION
WARNING: grampus is still under construction, so it is still missing some tests and features.

## TODO 

- **more tests**
- add a scheduler
- add more mutations
- make the grammar parser suck less
- write a good fuzzer

## Architecture and Documentation

see `doc` folder.

## Grammar Parser

parses a grammer in the form below. See `grammars` directory for examples.

```
S ::= 'a'S'b' | 'a' 'b'
# productions are marked by LHS ::= RHS 
# terminals are in single quotes
# non-terminal are all strings without single quotes
# if a space is to be kept, mark it with a "^" character
```
If you want to keep a space in the grammar, use '#'. The parser
will replace this with a space (0x20).

## Grammar 

takes the parsed grammar and generates random syntax trees. 
these trees are productions, with the leaves being terminals.
this can be used to generate valid sentences from the grammar.
inputs in the form of strings are generated from these.

## Grammar Fuzzer

takes the inputs from Grammar and applies mutations. harnesses
the PUT and performs fuzzing showing statistics along the way.

### 0xca7
