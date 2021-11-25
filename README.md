# grampus
my crappy grammar fuzzer

## Grammar Parser

parses a grammer in the form below from a file

```
<START> ::= <EXPR> | ( <EXPR> )
<EXPR> ::= 0 | 1
```

## Grammar Generator

takes the parsed grammar and generates random syntax trees. 
these trees are productions, with the leaves being terminals.
this can be used to generate valid sentences from the grammar.

## Grammar Fuzzer

takes the syntax trees / inputs and applies mutations. harnesses
the PUT and performs fuzzing

### 0xca7
