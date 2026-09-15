# `dyn`

Use `dyn` when you want to pass around values that implement some trait, without caring about the concrete implementation.

Roughly similar to Go's interfaces in the sense that they're a fat pointer (pointer to value and pointer to a vtable for looking up the methods of the trait) on that particular type.
