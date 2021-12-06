# Grampus
a crappy grammar fuzzer

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
cargo run
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
