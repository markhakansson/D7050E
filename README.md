# simple-rustc

A basic compiler that compiles a very barebones Rust (-like) language. The tools included are a language parser, type checker, an intepreter (no compilation), an LLVM compiler. 

It was created for the compiler construction and formal languages (D7050E) course at LTU. See the [REPORT.md](REPORT.md) for details on the complete language grammar and language proofs.

## Requirements

* Rust stable 1.38+ (might work on earlier versions)
* LLVM 8.0 (for compilation)

## Running tests
Currently code coverage is very low. Tests has only been created for mostly the parser and then a few for the interpreter and type checker. To run the tests call:

```
cargo test
```

## Usage
Pull the latest version from git. Then add the following to your Cargo.toml project
```toml
[dependencies]
simple-rustc = { path = "/PATH/TO/SIMPLE-RUSTC"}
```
The functions that are available to use are the following
```rust
pub fn parse_program(input: &str) -> IResult<&str, Functions>
pub fn tc_program(fn_list: &mut Functions) -> TypeRes<Type> // type checker
pub fn eval_program(fn_tree: &mut Functions) -> EvalRes<Value> // interpreter
pub fn compile_program(fn_list: Functions) // LLVM
```
The way programs are interpreted and compiled it assumes that there is always a main function in the program.

## Issues
There are a few bugs in the type checker were errors are thrown even though the interpreter will correctly evaluate the program. In the case of when evaluating binomial expressions with both boolean and integer expressions might result in an error.

The LLVM compiler is not complete. It does not support function arguments for example.

## License
Licensed under the MIT license. See [LICENSE.md](LICENSE.md) for details.