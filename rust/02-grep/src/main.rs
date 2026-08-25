use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

// Very simple "grep" implementation for scanning a single file for a byte pattern
//
// TODO
//
// [x] Basic matching
// [x] Match position reporting
// [x] Print matching lines
// [x] Add highlighting to matches
// [ ] Support multiple files
// [ ] Write tests for split()
//
// I think this would be enough to get the gist, there are obvious improvements we could make:
//   - optimised matching (KMP, BM, probably some SIMD algorithms to investigate)
//   - support basic regular expressions
//   - print surrounding context
//
// We'll see...

const HL_ON: &str = "\x1b[31m";
const HL_OFF: &str = "\x1b[0m";

#[derive(Parser, Debug)]
#[command(name = "grep")]
#[command(about = "Poor man's grep")]
struct Args {
    #[arg()]
    pattern: String,

    #[arg()]
    path: String,
}

struct Chunk {
    range: Range<usize>,
    is_match: bool,
}

fn split(pattern: &str, line: &str) -> (Vec<Chunk>, i32) {
    let mut out: Vec<Chunk> = Vec::new();

    let mut matches = 0;
    let mut pos = 0;

    // i know there are probably better ways (e.g. line.match_indices()) but for learning
    // purposes i wanted to do this by hand.
    'outer: while pos < line.len() {
        match line[pos..].find(pattern) {
            Some(ix) => {
                if ix > 0 {
                    out.push(Chunk {
                        range: pos..pos + ix,
                        is_match: false,
                    })
                }
                out.push(Chunk {
                    range: pos + ix..pos + ix + pattern.len(),
                    is_match: true,
                });
                pos += (ix + pattern.len());
                matches += 1;
            }
            None => {
                out.push(Chunk {
                    range: pos..line.len(),
                    is_match: false,
                });
                break 'outer;
            }
        }
    }

    (out, matches)
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

    // open file
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);
    let mut line_buf = String::new();
    let mut line_no = 0;

    'outer: loop {
        line_buf.clear();
        line_no += 1;
        match reader.read_line(&mut line_buf) {
            Ok(size) => {
                if size == 0 {
                    break 'outer;
                }
                let (chunks, matches) = split(&args.pattern, &line_buf);
                if matches > 0 {
                    print!("{}:{}:", &args.path, line_no);
                    for chunk in chunks {
                        if chunk.is_match {
                            print!("{HL_ON}");
                            print!("{}", &line_buf[chunk.range]);
                            print!("{HL_OFF}");
                        } else {
                            print!("{}", &line_buf[chunk.range]);
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
