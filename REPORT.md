# Report
This is the report for the Compiler construction and formal languages (D7050E) course at Lulea Technology at University. 

Some of the proofs are not entirely correct. But hopefully they convey the idea at least!

## Repo
https://github.com/markhakansson/D7050E

## Syntax

### EBNF GRAMMAR:

```ebnf
program = { function };

function = "fn", var, params, return_type, block;

params = "(", { param { , param } }, ")";

param = var, ":", type;

return_type = "->", type;

type = "i32" | "bool" | "void";

block = "{", { lhs, ";" }, "}";

lhs = let  | var_op | if | while | func_call | return; 

expr = num | var | bool | bin_op | func_call | parens;

num = ? Rust i32 ?;

var = ? Rust String ?;

bool = "true" | "false";

let = "let", var, ":", type, expr; 

parens = "(", expr, ")";

bin_op = math_expr | bool_expr | relation_expr;

math_expr = expr, math_token, expr;

bool_expr = expr, bool_token, expr;

relation_expr = expr, relation_token, expr;

math_token = "-" | "+" | "*" | "/";

bool_token = "&&" | "||";

relation_token = "<" | ">" | "==" | "!=";

var_op = var, var_token, expr;

var_token = "=" | "+=" | "-=" | "*=";

if = "if", parens, block;

while = "while", parens, block;

func_call = var, args;

args = "(", { arg { , arg } }, ")";

arg = num | var | bool;

return = "return", [ expr ];

```
### EBNF examples:
The following program uses all the grammar rules above.

```rust
fn test_one(b: bool) -> i32 {
    let a: i32 = 0;
    if b {
        a = 50;
    };

    return a;
}

fn test_two(a: i32, b: bool, c: i32) -> i32 {
    let variable: bool = (a == c) && b;
    let num: i32 = 0;
    while variable {
        num += 1;
        variable = false;
    };
    return num;    
}

// Explicit return types on all functions needed
fn main() -> () {
    let a: i32 = test_one(true);
    let b: i32 = test_two(a, true, 50);
    let c: i32 = a + b;
}

```

### Requirements
The parser has been implemented using the *nom* crate for Rust. The parser does not handle precedence for arithmetic or boolean expressions. But has priority for parenthesized expressions. There is no support for location information should an error occur during parsing. There is error recovery inside the program but does not output any useful information upon error.

All code has been written by me with inspiration from the professor Per Lindgrens's parser example on GitLab. Which is mostly the way __map__ function was used with *nom*.

## Semantics

 ### Structural Operational Semantics

$n \in num$

$b \in bool$

$x \in var$

$e \in expr$

function call (void):

$\lang fc, \sigma \rang \Downarrow void$

function call (i32):

$\lang fc, \sigma \rang \Downarrow n$

function call (bool):

$\lang fc, \sigma \rang \Downarrow b$

binomial operation (addition):

$\frac{\lang e1,\sigma \rang \Downarrow n1 \lang e2,\sigma \rang \Downarrow n2}{\lang e1 \text{ + }e2 \rang \Downarrow n1 \text{ plus } n2}$

left-hand side expression sequence (keywords):

$\frac{\lang lhs1,\sigma \rang \Downarrow \sigma' \lang lhs2,\sigma' \rang \Downarrow \sigma''}{\lang lhs1;lhs2,\sigma \rang \Downarrow \sigma''}$

let:

$\frac{}{\lang x := n,\sigma \rang \Downarrow \sigma[x := n]}$

while-false:

$\frac{\lang b,\sigma \rang \Downarrow false}{\lang \textit{while b do block},\sigma \rang \Downarrow \sigma}$

while-true:

$\frac{\lang b,\sigma \rang \Downarrow true \lang block,\sigma \rang \Downarrow \sigma' \lang \textit{while b do block},\sigma' \rang \Downarrow \sigma''}{\lang \textit{while b do block},\sigma \rang \Downarrow \sigma''}$

if:

$\frac{\lang b,\sigma \rang \Downarrow true \lang block,\sigma \rang \Downarrow \sigma'}{\lang \textit{if b then block},\sigma \rang \Downarrow \sigma'}$

### Explanation
The example program starts from "main" and assigns the variable "a" to an i32 value returned from a function call to "test_one" with the argument "true".

In the "test_one" function a variable "a" is assigned to "0":

$\frac{}{\lang a := 0,\sigma \rang \Downarrow \sigma[a := 0]}$

then the if-statement is checked with the value "true":

$\frac{\lang b,\sigma \rang \Downarrow true \lang block,\sigma \rang \Downarrow \sigma'}{\lang \textit{if b then block},\sigma \rang \Downarrow \sigma'}$

this sets the value to "a" to be "50":

$\frac{}{\lang a := 50,\sigma \rang \Downarrow \sigma[a := 50]}$

then variable "a" is returned

$\lang \textit{test\_one}, \sigma \rang \Downarrow 50$

In the "main" function "a" is set to the returned value:


$\frac{}{\lang a := 50,\sigma \rang \Downarrow \sigma[a := 50]}$

After that variable "b" is declared as an i32 value to the return value from "test_two" with the values "a", "true" and "50".

In the "test_two" function variable "variable" is declared as a boolean expression. In the boolean expression the paranthesised expressions is first evaluated:

$\frac{\lang a, \sigma \rang \Downarrow 50 \lang c, \sigma \rang \Downarrow 50}{\lang \textit{a equals c}, \sigma \rang \Downarrow true}$

then with the rest:

$\frac{\lang \textit{a and c},\sigma \rang \Downarrow true \lang b,\sigma \rang \Downarrow true}{\lang \textit{(a equals c) and b} \rang \Downarrow true}$

Thus the variable "variable" is set to the value "true". And the variable "num" to "0".

Then the while-loop starts with the condition "variable". Since "variable" is set to true the loop starts:

$\frac{\lang variable,\sigma \rang \Downarrow true \lang block,\sigma \rang \Downarrow \sigma' \lang \textit{while variable do block},\sigma' \rang \Downarrow \sigma''}{\lang \textit{while variable do block},\sigma \rang \Downarrow \sigma''}$

Inside the "block" the variable "num" is incremented by + "1" 

$\frac{}{\lang num := num + 1,\sigma \rang \Downarrow \sigma[num := num + 1]}$

and directly after "variable" is set to "false":

$\frac{}{\lang variable := false,\sigma \rang \Downarrow \sigma[variable := false]}$ 

since the variable "variable" is false the while-loop
will not execute the inner block and the state is unchanged:

$\frac{\lang variable,\sigma \rang \Downarrow false}{\lang \textit{while variable do block},\sigma \rang \Downarrow \sigma}$

Finally "num" is returned which has the value of "1".
Back in the "main" function "b" is set to the return value of "test_two" which was "1". And then another variable "c" is declared to "a + b". Which will be "51".

The current interpreter supports this SOS and can execute programs according to it. Should the program fail, errors will be returned with information about what went wrong.

## Type Checker

### Type Checking Rules
let (bool):

$\frac{\lang x,\sigma \rang \Downarrow bool \lang b,\sigma \rang \Downarrow bool}{\lang x := b,\sigma \rang \Downarrow \sigma[x := b]}$

let (i32):

$\frac{\lang x,\sigma \rang \Downarrow i32 \lang n,\sigma \rang \Downarrow i32}{\lang x := n,\sigma \rang \Downarrow \sigma[x := n]}$

binomial operation (math):

$\frac{\lang expr1,\sigma \rang \Downarrow i32 \lang expr2,\sigma \rang \Downarrow i32}{\lang \textit{ expr1 math\_token }expr2 \rang \Downarrow i32}$

binomial operation (boolean):

$\frac{\lang expr1,\sigma \rang \Downarrow bool \lang expr2,\sigma \rang \Downarrow bool}{\lang \textit{ expr1 bool\_token }expr2 \rang \Downarrow bool}$

binomial operation (relational):

$\frac{\lang expr1,\sigma \rang \Downarrow bool \lang expr2,\sigma \rang \Downarrow bool}{\lang \textit{ expr1 relation\_token }expr2 \rang \Downarrow bool}$

$\frac{\lang expr1,\sigma \rang \Downarrow i32 \lang expr2,\sigma \rang \Downarrow i32}{\lang \textit{ expr1 relation\_token }expr2 \rang \Downarrow bool}$

if:

$\frac{\lang condition,\sigma \rang \Downarrow bool}{\lang \textit{if condition do block} \rang}$

while:

$\frac{\lang condition,\sigma \rang \Downarrow bool}{\lang \textit{while condition do block} \rang}$

### Examples
Let's assign a new variable that is of type "i32" to an i32 values:
```rust
let a: i32 = 32;
```
The type rule is as follows:
$\frac{\lang a,\sigma \rang \Downarrow i32 \lang n,\sigma \rang \Downarrow i32}{\lang x := n,\sigma \rang \Downarrow \sigma[x := n]}$

For a relational operation such as
```rust
let b: bool = 30 > 10;
```
it will follow the following type rule:

$\frac{\lang 30,\sigma \rang \Downarrow i32 \lang 10,\sigma \rang \Downarrow i32}{\lang 30 \textit{ > }10 \rang \Downarrow bool}$

For an if-statement:

```rust
let b: bool = true;
if b {
    // do block
};
```
which will follow the type rule:

$\frac{\lang b,\sigma \rang \Downarrow bool}{\lang \textit{if condition do block} \rang}$

For while-statements:
```rust
let b: bool = true;
while b {
    // do block
}
```

and the type rule:

$\frac{\lang b,\sigma \rang \Downarrow bool}{\lang \textit{while condition do block} \rang}$

The implemented type checker follows the rules above and should it find that there is a type mistmatch, errors will be returned. The current error implementation does not stack, only the first found error will be sent to the terminal. It does however send some information on where the error occured.

## Borrow Checker
The borrow checker should check whether the variable is a mutable or unmutable borrow. If it's unmutable the program should not be able to change the value that the variable holds. If it's mutable the borrow checker should check that the mutable borrow does not occur somewhere else, so that the variable can't be written to at the same time.

## LLVM

## Overal course goals and learning outcomes.
I have improved my knowledge in Rust. As well as gained a basic understanding on how the Rust features such as borrows and type checks works. I've learnt how to create my own parser and then how to interpret an AST. 