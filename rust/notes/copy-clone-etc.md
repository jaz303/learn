# `Copy` trait

Placed on types that are stored on the stack; causes trivial copy rather than a move.

Can't annotate type with `Copy` trait if the type, or any of its parts, implement `Drop`.

Default types with `Copy` trait:

  - all integers
  - all floats
  - `bool`
  - `char`
  - tuples, if they only contain types that implement `Copy`

