# Error Trait

Must implement `Debug` and `Display` traits.

Additionally, methods exist for:

  - `source()` - lower-level source of error
  - `description()` - deprecated - use `Display`
  - `cause()` - deprecated - use `source()`
  - `provide()` - "Provides type-based access to context intended for error reports."
