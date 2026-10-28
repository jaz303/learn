use base64::engine::general_purpose::STANDARD;
use base64::write::EncoderWriter;
use std::cmp::min;
use std::io::{self, Stdout, Write};
use tiny_skia::Pixmap;

pub struct KittyRenderer {
    output: Stdout,
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
    pub fn new(output: Stdout) -> Self {
        Self {
            output,
            last_image_id: 0,
            tmp: Vec::new(),
        }
    }

    pub fn draw(&mut self, image: &Pixmap) -> io::Result<()> {
        let image_id = self.last_image_id + 1;
        let width = image.width();
        let height = image.height();

        self.tmp.clear();
        {
            let mut encoder = EncoderWriter::new(&mut self.tmp, &STANDARD);
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

        // must be a multiple of 4
        let data_bytes_per_chunk = 4032;

        if self.tmp.len() <= data_bytes_per_chunk {
            // single chunk
            write!(self.output, ";")?;
            self.output.write_all(&self.tmp)?;
            write!(self.output, "\x1b\\")?;
        } else {
            // multiple chunks

            // write the first chunk as part of the first (header) message...
            write!(self.output, ",m=1;")?;
            self.output.write_all(&self.tmp[0..data_bytes_per_chunk])?;
            write!(self.output, "\x1b\\")?;

            // ...then the remaining chunks
            let mut to_write = &mut self.tmp[data_bytes_per_chunk..];
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
