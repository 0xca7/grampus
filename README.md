# grampus
my crappy grammar fuzzer

## TODO 

- add a scheduler
- add more mutations
- make the grammar parser suck less
- write a good fuzzer

## Architecture



## Grammar Parser

parses a grammer in the form below from a file

```
START = <EXPR> | ( <EXPR> )
EXPR = 0 | 1
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
