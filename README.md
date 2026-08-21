# Borelang - Interpreted language written in rust
> NOTE: Still in very early development, this might change alot.


## Design Goals
* Rust-like syntax with high-level, python/GDScript-like expressiveness

## Current Features
* Constant and Mutable Variables
* if and for Statements
* Lightweight built-ins library (not fully)
* Basic error handler
* Explicit/Any types

## Code Example
```
# prints "Hello, world!"
greet("world")

fn greet(who: str) {
  println("Hello, ", who, "!")
}
```

## Devlogs / Notes
* **Contributions**: Currently not accepting outside pull requests, though feel free to suggest features!