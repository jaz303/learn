use base64::engine::general_purpose::STANDARD;
use base64::write::EncoderWriter;
use std::cmp::min;
use std::error::Error;
use std::io::{self, BufRead, BufReader, Read, Stdin, Stdout, Write};
use tiny_skia::{Color, Pixmap};

pub struct KittyRenderer {
    input: Stdin,
    output: Stdout,

    // origin is cell coordinates
    origin_x: u16,
    origin_y: u16,
    width: i32,
    height: i32,

    last_image_id: u32,

    tmp: Vec<u8>,
}

impl Drop for KittyRenderer {
    fn drop(&mut self) {
        let _ = self.delete_prev();
        let _ = self.output.flush();
    }
}

impl KittyRenderer {
    pub fn new(input: Stdin, output: Stdout, x: u16, y: u16) -> Self {
        Self {
            input,
            output,
            origin_x: x,
            origin_y: y,
            width: -1,
            height: -1,
            last_image_id: 0,
            tmp: Vec::new(),
        }
    }

    // // call this whenever crossterm detects a resize event;
    // // we communicate with Kitty to find out what the new
    // // image size is
    // pub fn resync(&mut self) -> Result<(i32, i32), Box<dyn Error>> {
    //     self.output.write_all(b"\x1b[14t")?;
    //     self.output.flush()?;
    //
    //     // to set the type explicitly, do this:
    //     // let mut hdr = [0u8; 8];
    //     // but Rust performs type inference over the entire function body
    //     // so this actually works fine...
    //     let mut hdr = [0; 8];
    //
    //     // read the header
    //     self.input.read_exact(&mut hdr[0..4])?;
    //     if hdr[0] != 0x1b || hdr[1] != b'[' || hdr[2] != b'4' || hdr[3] != b';' {
    //         eprintln!("invalid escape sequence received")
    //     }
    //
    //     let mut br = BufReader::new(&self.input);
    //
    //     self.tmp.clear();
    //     let size = br.read_until(b';', &mut self.tmp)?;
    //     let height_str = str::from_utf8(&self.tmp[0..size - 1])?;
    //     let height: i32 = height_str.parse()?;
    //
    //     self.tmp.clear();
    //     let size = br.read_until(b't', &mut self.tmp)?;
    //     let width_str = str::from_utf8(&self.tmp[0..size - 1])?;
    //     let width: i32 = width_str.parse()?;
    //
    //     self.width = width;
    //     self.height = height;
    //
    //     Ok((width, height))
    // }

    pub fn draw(&mut self, image: &Pixmap) -> io::Result<()> {
        let image_id = self.last_image_id + 1;
        let width = image.width();
        let height = image.height();

        // Encode the image to base64 in memory
        let mut encoded = Vec::new();
        {
            let mut encoder = EncoderWriter::new(&mut encoded, &STANDARD);
            encoder.write_all(image.data())?;
            encoder.finish()?;
        }

        // Header
        // _G
        // a=T  - transmit and display
        // f=32 - 32-bit RGBA data (TODO: benchmark 24-bit and PNG alternatives)
        // t=d  - direct data transfer (TODO: benchmark SHM)
        // C=1  - don't move cursor
        // i    - image ID
        // s,v  - image width, height
        write!(
            self.output,
            "\x1b_Ga=T,f=32,t=d,C=1,i={image_id},s={width},v={height}"
        )?;

        let data_bytes_per_chunk = 4032;

        if encoded.len() <= data_bytes_per_chunk {
            // single chunk
            write!(self.output, ";")?;
            self.output.write_all(&encoded)?;
            write!(self.output, "\x1b\\")?;
        } else {
            // multiple chunks

            // write the first chunk as part of the first (header) message...
            write!(self.output, ",m=1;")?;
            self.output.write_all(&encoded[0..data_bytes_per_chunk])?;
            write!(self.output, "\x1b\\")?;

            // ...then the remaining chunks
            let mut to_write = &mut encoded[data_bytes_per_chunk..];
            while !to_write.is_empty() {
                if to_write.len() <= data_bytes_per_chunk {
                    write!(self.output, "\x1b_Gm=0;")?;
                } else {
                    write!(self.output, "\x1b_Gm=1;")?;
                }
                let cnt = min(data_bytes_per_chunk, to_write.len());
                self.output.write_all(&to_write[0..cnt])?;
                write!(self.output, "\x1b\\")?;
                to_write = &mut to_write[cnt..];
            }
        }

        self.delete_prev()?;
        self.last_image_id = image_id;

        self.output.flush()
    }

    fn delete_prev(&mut self) -> Result<(), io::Error> {
        if self.last_image_id > 0 {
            let del_id = self.last_image_id;
            write!(self.output, "\x1b_Ga=d,d=I,i={del_id}\x1b\\")
        } else {
            Ok(())
        }
    }
}
