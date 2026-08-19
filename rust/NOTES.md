# NOTES

## Ownership

  - value has owner
  - can only be one owner at a time
  - when the owner goes out of scope, the value is dropped; memory is returned

types of known size can be stored on the stack.

when value goes out of scope, drop() is called (if type has Drop trait)

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!");
```

so in the above, `s2 = s1` causes `s1` to become invalid.
this process is called a "move".
no automatic deep copying of data.

clone() if you do want a copy

Copy trait can be used for types that are stored on the stack (like integers). If a type
implements the Copy trait, variables are not moved, but are trivially copied.
Copy can't be used if the type, or any of its parts implement the Drop trait.

Default Copy types: integers, bools, floats, char, and tuples of the same

passing variable to a function will move or copy, just like assignment.
and returning can too! see:

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

# BORROWING

So you can borrow a reference and give it back!

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // referencing

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

So the reference doesn't OWN it.
Creating a reference = borrowing.
You can't modify something you have a reference to - you need a MUTABLE REFERENCE (`&mut`)

Restriction - if you use mutable references, there can be NO OTHER REFERENCES to that value.