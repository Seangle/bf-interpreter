# BF Interpreter

An interpreter for the esoteric programming language called "Brainfuck"[1]. Aptly
named for its confusing syntax.

The language consists of 8 instructions which operate on a continous array of
memory through the use of an address pointer:

operator | description
--- | ---
`.` | Output value at current address
`,` | Input value at current address
`>` | Move to next address
`<` | Move to previous address
`+` | Increment value at current address
`-` | Decrement value at current address
`[` | Skip all contents to next `]` if value at current address is 0, otherwise continue 
`]` | Return to previous `[` if value at current address is not 0, otherwise continue


## Building

Building the software is done with Rust's `cargo`. Use `cargo build` in the
project directory to build.

## Running

Running the `bfi` interpreter can be done after building with cargo. To run the
software, either use the executable directly: `bfi <path_to_file>`
or through cargo: `cargo run <path_to_file>`

Alternitavely, the `bf` library can be used as a library by including it in a
toml file:

```
[dependencies]
bf = { path = "./<my_bf_path>" }
```

## License

See `LICENSE.txt`.

## References

1. [Brainfuck - Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)