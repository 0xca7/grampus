# Grampus
## a (crappy) grammar fuzzer

A grammar fuzzer has knowledge of a formal grammar and uses that grammar to generate valid inputs.
For example, a grammar fuzzer may know the grammar to produce a valid URL. This valid URL is then
mutated and used for fuzzing. For example, it could be used to fuzz an URL parser.

Grampus is a grammar fuzzer that reads a grammar from a file, it then generates a corpus using
that grammar and uses that corpus for fuzzing. 

![Grampus](doc/grampus.png)

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

## Try it Out

For a description read the pdf in the `doc` folder.

```
./bootstrap.sh

TRY IT OUT:
cargo run -- --fuzz-target fuzz_target/example_target --grammar-file grammars/json.txt --start-symbol JSON

USAGE:
cargo run -- --fuzz-target <fuzz_target> --grammar-file <grammar_file> --start-symbol <start_symbol>

```

## Grammar 

grammars are entered in the form below. 
See `grammars` directory for examples.

```
S ::= 'a'S'b' | 'a' 'b'
# productions are marked by LHS ::= RHS 
# terminals are in single quotes
# non-terminal are all strings without single quotes
# if a space is to be kept, mark it with a "^" character
```
If you want to keep a space in the grammar, use '^'. The parser
will replace this with a space (0x20).

### 0xca7
