use clap::Parser;
use std::fs::File;
use std::io::Read;

// Very simple "grep" implementation for scanning a single file for a byte pattern
//
// TODO
//
// [x] Basic matching
// [ ] Match position reporting
// [ ] Print matching lines
// [ ] Add highlighting to matches
// [ ] Support multiple files
//
// I think this would be enough to get the gist, there are obvious improvements we could make:
//   - optimised matching (KMP, BM, probably some SIMD algorithms to investigate)
//   - support basic regular expressions
//   - print surrounding context
//
// We'll see...

#[derive(Parser, Debug)]
#[command(name = "grep")]
#[command(about = "Poor man's grep")]
struct Args {
    #[arg()]
    pattern: String,

    #[arg()]
    path: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    // can't match against an empty pattern
    if args.pattern.len() == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "pattern cannot be blank",
        ));
    }

    // we want to match bytes, so convert args.pattern to bytes - we own them now.
    let pattern = args.pattern.into_bytes();

    // ring buffer
    // 2nd line is more idiomatic (and slightly different - it actually fills it with elements
    // whereas i think the first just creates an empty vector with the given capacity.
    // let ring: Vec<u8> = Vec::with_capacity(args.pattern.len());
    let mut ring = vec![0; pattern.len()];

    // open file
    let mut file = File::open(args.path)?;

    // seed the buffer with the initial bytes (up to pattern length - 1)
    match file.read_exact(&mut ring[0..pattern.len() - 1]) {
        Ok(()) => { /* bytes read OK - nothing to do */ }
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
            // if we couldn't read len(pattern) bytes, there can be no match, so exit early
            return Ok(());
        }
        Err(err) => return Err(err),
    }

    // borrow a mutable reference to ring buffer's backing store
    // 2nd form is more idiomatic... leaving the first in as a comment to remind me that you
    // can borrow an arbitrary slice view of the backing store.
    // let buffer = &mut ring[0..pattern.len()];
    let buffer = &mut ring;

    // ring buffer write pointer
    let mut wp: usize = pattern.len() - 1;

    // hereafter the algorithm is simply
    //   - read next byte into ring buffer
    //   - compare against head of buffer
    loop {
        let mut b = [0u8; 1];
        let nread = file.read(&mut b)?;
        if nread == 0 {
            break;
        }
        buffer[wp] = b[0];
        wp = (wp + 1) % pattern.len();

        // wp is now sitting at the first byte to match
        let mut is_match = true;
        for i in 0..pattern.len() {
            if buffer[(wp + i) % pattern.len()] != pattern[i] {
                is_match = false;
                break;
            }
        }

        if is_match {
            println!("got a match!");
        }
    }

    Ok(())
}
