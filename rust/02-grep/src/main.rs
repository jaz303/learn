use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

// Very simple "grep" implementation for scanning a single file for a byte pattern
//
// TODO
//
// [x] Basic matching
// [x] Match position reporting
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

#[derive(Debug, Copy, Clone)]
struct Pos {
    line: usize,
    col: usize,
}

enum PosTrackerNewlineState {
    Out,
    In,
}

struct PosTracker {
    pos: Pos,
    state: PosTrackerNewlineState,
}

impl PosTracker {
    fn new() -> PosTracker {
        PosTracker {
            pos: Pos { line: 0, col: 0 },
            state: PosTrackerNewlineState::In,
        }
    }

    // push a byte into the tracker
    // returns the character's position
    fn push(&mut self, b: u8) -> Pos {
        let out = self.pos;

        match self.state {
            PosTrackerNewlineState::Out => {
                if b == b'\n' {
                    self.pos.line += 1;
                    self.pos.col = 0;
                } else if b == b'\r' {
                    self.pos.line += 1;
                    self.pos.col = 0;
                    self.state = PosTrackerNewlineState::In;
                } else {
                    self.pos.col += 1;
                }
            }
            PosTrackerNewlineState::In => {
                if b != b'\n' {
                    self.pos.col += 1;
                }
                self.state = PosTrackerNewlineState::Out;
            }
        }

        out
    }
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

    let mut pos_ring = vec![Pos { line: 0, col: 0 }; pattern.len()];

    // open file
    let mut file = File::open(args.path)?;

    // seed the buffer with the initial bytes (up to pattern length - 1)
    // what we're doing here is borrowing a mutable slice from the vector
    match file.read_exact(&mut ring[0..pattern.len() - 1]) {
        Ok(()) => { /* bytes read OK - nothing to do */ }
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
            // if we couldn't read len(pattern) bytes, there can be no match, so exit early
            return Ok(());
        }
        Err(err) => return Err(err),
    }

    let mut pos_tracker = PosTracker::new();
    for i in 0..pattern.len() - 1 {
        pos_ring[i] = pos_tracker.push(ring[i]);
    }

    // borrow a mutable reference to ring buffer's backing store
    // 2nd form is more idiomatic... leaving the first in as a comment to remind me that you
    // can borrow an arbitrary slice view of the backing store.
    // let buffer = &mut ring[0..pattern.len()];
    let buffer = &mut ring[..];

    // ring buffer write pointer
    let mut wp: usize = pattern.len() - 1;

    // create a buffered reader, for efficiency!
    let mut reader = BufReader::new(file);

    // hereafter the algorithm is simply
    //   - read next byte into ring buffer
    //   - compare against head of buffer
    loop {
        let mut b = [0u8; 1];
        let nread = reader.read(&mut b)?;
        if nread == 0 {
            break;
        }

        buffer[wp] = b[0];
        pos_ring[wp] = pos_tracker.push(b[0]);
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
            let match_pos = pos_ring[wp];
            println!("got a match at {},{}", match_pos.line, match_pos.col);
        }
    }

    Ok(())
}
