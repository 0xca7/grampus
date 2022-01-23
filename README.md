# Grampus
## a (crappy) grammar fuzzer

A grammar fuzzer has knowledge of a formal grammar and uses that grammar to generate valid inputs.
For example, a grammar fuzzer may know the grammar to produce a valid URL. This valid URL is then
mutated and used for fuzzing. For example, it could be used to fuzz an URL parser.

Grampus is a grammar fuzzer that reads a grammar from a file, it then generates a corpus using
that grammar and uses that corpus for fuzzing. 

![Grampus](doc/grampus.png)

## Dependencies / Crates
Grampus uses:

```
regex = "1"
colored = "2.0.0"
clap = "2.34.0"
```

Thanks to the authors of these crates, you rock!

## TODO 

- **more tests**
- add more mutations
- make the grammar parser suck less
- write a good fuzzer

## Architecture and Documentation

see `doc` folder.

## Try it Out

For a description read the pdf in the `doc` folder.

You can either have Grampus run in `fuzz` or in `gen` mode.

Either way, the first thing you need to do is run the `bootstrap` script:

```
chmod +x bootstrap.sh
./bootstrap.sh
```

Now you can start fuzzing the example target:
```
# long syntax
cargo run -- --mode fuzz --fuzz-target fuzz_target/example_target --grammar-file grammars/json.txt --start-symbol JSON

# short syntax
cargo run -- -m fuzz -t fuzz_target/example_target -g grammars/json.txt -s JSON
```

In general, fuzzing looks like this:
```
USAGE:
cargo run -- --fuzz-target <fuzz_target> --grammar-file <grammar_file> --start-symbol <start_symbol>
cargo run -- -m fuzz -t <fuzz_target> -g <grammar_file> -s <start_symbol>
```

... or you can run grampus in the `gen` mode, which just generates some inputs for fuzzing 
you can use with a different (actually good) fuzzer, for instance AFL. The ouput is stored in
`corpus/`
```
cargo run -- --mode gen --grammar-file <grammar_file> --start-symbol <start_symbol>

EXAMPLE:
cargo run -- --mode gen --grammar-file url.txt --start-symbol URL
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

If you want to insert a newline, use `\n`, this will be replaced 
with a newline for you (0x0a).

For an example, see `grammars/ini.txt`.

---

Now go and fuzz the planet :^)

### 0xca7
