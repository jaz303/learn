Reference: https://dev.to/sgchris/the-debug-vs-display-traits-whats-the-difference-1l58

## Debug Trait

For developers, debugging etc. Output should be detailed, unambiguous.

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

println!("{:?}", p); // use debug formatting; can use {:#?} for pretty-printing (multi-line)
```

## Display Trait

For end users; cleaner, human-friendly representation for UI and/or logs.

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// implement it manually
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("{}", p); // Display formatting
}
```
