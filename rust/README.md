# rust

I'm learning Rust. Just grinding small useless projects until it becomes automatic.

## Completed Projects

### `dupes`

Find duplicate files in directory; `clap` CLI, parallel hashing, recursive option.

## Ideas

  - Networking:
    - some sort HTTP API client
    - some sort of HTTP server (API with axum + sqlx) (extractors, shared state, compile-type-checked SQL)
    - chat - Tokio, broadcast channels, select!, backpressure 
    - concurrent uptime reader - JoinSet, timeouts, retries, Send + 'static
    - port scanner
    - DNS resolution
    - HTTP proxy
  - coreutils:
    - grep (recursive search with memmap)
    - find
  - bounded web crawler (channels vs Arc<Mutex<_>>, semaphore, graceful shutdown)
  - CLI tools
    - `todo` - CLI for managing a TODO file (non-interactive)
  - TUI
    - toy shell
    - kilo-style text editor (crossterm) - event loop, undo stack, buffer 
    - system monitor with ratatui
    - Roguelike
  - Port [`spinup`](https://github.com/jaz303/spinup)
  - Data: CSV query engine
  - mini database engine
  - Games: terminal roguelike, CHIP-8 emulator, Game Boy emulator
  - simple VCS?
    - from Git: hash-object, cat-file, commit-tree (zlib, byte-level parsing, content-address storage)
  - KVS will write-ahead log
    - file I/O, serde, compaction, crash consistency
  - Lisp/Scheme interpreter
    - enums/pattern matching, Vec indices as an arena instead of Rc<RefCell<Node>>
  - embedded project (RP2040 etc, no_std, no allocator, async without a heap)
  - hand written JSON parser - everything borrows from one &str
  - static site generator - pulldown-cmark, tera, notify (for live reload)
  - log analyzer - iterator chains under memory pressure
  - FFI wrapper around a C library