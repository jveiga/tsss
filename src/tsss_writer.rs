use std::io::{BufWriter, Write};

use chrono::{DateTime, Local};

// TODO
// * implement as enum?
pub struct TS<W: Write> {
    w: BufWriter<W>,
    start: DateTime<Local>,
    last: DateTime<Local>,
    is_first: bool,
    v: Vec<u8>,
}

impl<W: Write> TS<W> {
    pub fn new(start: DateTime<Local>, inner: W) -> Self {
        Self {
            start,
            last: start,
            is_first: true,
            v: Vec::with_capacity(8 * 1024),
            w: BufWriter::new(inner),
        }
    }
}

impl<W> Write for TS<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for bt in buf.iter() {
            // found newline
            if *bt != b'\n' {
                self.v.push(*bt);
                continue;
            }
            // first print
            let now = Local::now();
            if self.is_first {
                let _ = self.w.write(
                    format!(
                        "{:?}ms ",
                        now.signed_duration_since(self.start).num_milliseconds()
                    )
                    .as_bytes(),
                )?;
                self.is_first = false;
            } else {
                // Since start
                let _ = self.w.write(
                    format!(
                        "{:?}ms ",
                        now.signed_duration_since(self.start).num_milliseconds()
                    )
                    .as_bytes(),
                )?;
                // since last
                let _ = self.w.write(
                    format!(
                        "{:?}ms ",
                        now.signed_duration_since(self.last).num_milliseconds()
                    )
                    .as_bytes(),
                )?;
            }
            let _ = self.w.write(&self.v)?;
            let _ = self.w.write("\n".as_bytes())?;
            self.v.clear();
            self.last = Local::now();
        }
        self.w.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}
