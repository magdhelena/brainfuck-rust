# brainfuck-rust
Brainfuck interpreter made to practice Rust 🦀

## Usage
It can read the bf code both as a string argument or from an input file with the `-f` flag:
```
cargo run "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
```
or
```
cargo run -- -f hello_world.b
```
where `hellow_world.b` contains the bf code above.

[Here](https://brainfuck.org/) you can find some example programs by Daniel B. Cristofani.
