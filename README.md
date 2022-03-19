# Prime-RS

Command line utilities to find prime numbers.
```nprime [number]``` finds the next prime number starting at the given argument, ```pprime [number]``` finds the previous prime number to the given argument.

## How to build?

You'll need to have a [rust toolchain installed](https://www.rust-lang.org/tools/install). If you don't need the source code you can simply run ```cargo install prime-rs```.
If you prefer running it straight from source code, clone the repository, ```cd``` into the project directory and run ```cargo run --bin nprime 60017``` or ```cargo run --bin pprime 60017```. The result will be printed to ```stdout```.