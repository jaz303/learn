use clap::Parser;
use hex;
use sha2::digest::Output;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use rayon::prelude::*;

// dupes - find duplicate files in a directory tree

// TODO
// [x] index files by size (map -> vector of paths)
// [x] iterate over index, find sizes with > 1 entry, do hash-based comparison, report dupes
// [x] add a proper CLI - pull in a 3rd party library
// [x] option to walk recursively
// [x] make the hashing parallel
// [x] improve parallel hashing architecture

#[derive(Parser, Debug)]
#[command(name = "dupes")]
#[command(about = "Find duplicate files")]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long, default_value = "0")]
    concurrency: usize,
}

fn hash_file(path: &Path) -> Result<Output<Sha256>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; 8192];
    let mut digest = Sha256::new();
    loop {
        let nread = file.read(&mut buf)?;
        if nread == 0 {
            break; // EOF
        }
        digest.update(&buf[..nread]);
    }
    Ok(digest.finalize())
}

// dupes - find duplicate files in a directory
fn main() {
    let args = Args::parse();

    if args.concurrency > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.concurrency)
            .build_global()
            .unwrap();
    }

    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    // Stack of directories is of PathBuf - this is because PathBuf *owns* its contents
    // By contrast, &PathBuf would mean it was a vector of items that are borrowed/owned
    // elsewhere.
    let mut dir_stack: Vec<PathBuf> = Vec::new();

    // Seed the path stack with a PathBuf created from the input path
    // PathBuf::from does an allocation, ownership transfers back to us, then is
    // transferred to the vector.
    dir_stack.push(PathBuf::from(&args.path));

    while !dir_stack.is_empty() {
        let path = dir_stack.pop().unwrap();

        // As we read each entry, we become the owner of each returned entry
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap().path();

            // Create another entry
            // What happens with ownership here?
            //
            // path.join() takes &self as its first parameter, so it doesn't consume path.
            //
            // The second parameter, p, is a Path (and not a &Path), so it *is* consumed.
            //
            // The rules:
            // fn thing(x: T)       -> takes ownership
            // fn thing(x: &T)      -> borrows immutably
            // fn thing(x: &mut T)  -> borrows mutably
            //
            // let entry = path.join(entry);
            // NOTE: line above is commented out because the paths obtained by read_dir() are absolute
            // so there's no need for us to do a join. I'm leaving it in the code for the comments around
            // ownership.

            let metadata = entry.metadata().unwrap();
            if metadata.is_dir() {
                if args.recursive {
                    // Ownership of entry is transferred to dir_stack - this is fine because we
                    // restart the loop immediately without using entry again. If we remove the
                    // "continue" below, notice that it breaks
                    dir_stack.push(entry);
                }
                continue;
            }

            by_size
                .entry(metadata.len())
                .or_insert(Vec::new())
                .push(entry);
        }
    }

    let mut to_hash: Vec<PathBuf> = Vec::new();

    // This "consumes" the hashmap
    for (_, v) in by_size {
        if v.len() > 1 {
            for p in v {
                to_hash.push(p);
            }
        }
    }

    // so in ownership terms, what's happening here is:
    // to_hash is currently the owner of the vector.
    // v.into_par_iter() consumes v and yields ownership of its elements
    // the map callback receives ownership of the value (this means v.into_par_iter() *must* consume v,
    // because otherwise its elements couldn't remain in the vector)
    // finally, collect() takes ownership of each item from iterator and constructs Vec from them
    // end result: we have a vector contains values we fully own
    //
    // General rules:
    // iter()       -> borrow elements
    // iter_mut()   -> mutable borrow elements
    // into_iter()  -> consume collection, own elements
    //
    let results: Vec<Result<(PathBuf, Output<Sha256>), std::io::Error>> = to_hash
        .into_par_iter()
        .map(|path| {
            let hash = hash_file(path.as_path());
            match hash {
                Ok(hash) => { Ok((path, hash)) }
                Err(e) => {
                    println!("couldn't hash {}: {}", path.display(), e);
                    Err(e)
                }
            }
        })
        .collect();

    let mut by_digest: HashMap<Output<Sha256>, Vec<PathBuf>> = HashMap::new();
    for r in results {
        match r {
            Ok(hash) => {
                by_digest.entry(hash.1).or_insert(Vec::new()).push(hash.0)
            }
            Err(e) => {}
        }
    }

    for (hash, v) in by_digest {
        if v.len() <= 1 {
            continue;
        }
        println!("{} (n={})", hex::encode(hash), v.len());
        for path in v {
            println!("  - {}", path.display())
        }
    }
}
