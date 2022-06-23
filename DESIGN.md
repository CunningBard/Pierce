python like? or rust like?

name shall be "pierce" or py-rst -> python-rust 
### Questions to be answered:
- ~~should variables be immutable by default?~~
- ~~should syntax be rust like but simpler?~~
- low level data types? i32, i16, i8, f32?

### Agreed upon:
- variables shall be immutable by default
- types like python i.e `int` `str` `bool` `float`
- syntax shall be rust-like but simpler
- functions will be private(other files can't
access the function unless it is being told its public)
by default
- 

### Sample code
```
// import std; example code

let value -> int = 123;

let identifier -> int = value;

def func_name() -> bool {
    return True;
}

if identifier == value and func_name() {
    print("why you mix rust and python - engrish guy");
}

print(identifier); // output is 123
```

### What:
types:
- `int` is a datatype which holds number of any size
- `str` is a datatype which holds multiple characters
- `bool` is a datatype which represents true and false
- `float` is a datatype which holds a number of any size but has a decimal point

### Visualizer
```let d = 123;```
let identifier -> type = value;

### TODO
completely finish Lexer
start working on parser

do code gen

do auto type checking


# Pierce
A simple language inspired by the readability of python and the speed and features of rust

### formerly known as YAPL_rewrite
A re-write of yet another programming language, YAPL is a langauge that aims to be simple as python and really fast.

# WARNING! this is the dev branch

## Syntax
```
from std.out import *; // lets you print, throw error, and etc (not yet implemented)

let d -> str = "shiz";

for character in d {
    print(d, sep="");
}
print() // prints newline

// NOTE: syntax might change check design.md
```