use std::io::{BufWriter, Write};

use chrono::{DateTime, Local};

// TODO
// * add arguments? picoargs?

fn main() -> std::io::Result<()> {
    let start = Local::now();
    let stdout_handle = std::io::stdout().lock();
    let mut stdin_handle = std::io::stdin().lock();
    std::io::copy(
        &mut stdin_handle,
        &mut TS {
            start,
            last: start,
            is_first: true,
            v: Vec::with_capacity(8 * 1024),
            w: BufWriter::new(stdout_handle),
        },
    )?;

    Ok(())
}

// TODO
// * implement as enum?
struct TS<W: Write> {
    w: BufWriter<W>,
    start: DateTime<Local>,
    last: DateTime<Local>,
    is_first: bool,
    v: Vec<u8>,
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
                let _ = self.w.write(&self.v)?;
                let _ = self.w.write("\n".as_bytes())?;
                self.v.clear();
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
                let _ = self.w.write(&self.v)?;
                let _ = self.w.write("\n".as_bytes())?;
                self.v.clear();
            }
            self.last = Local::now();
        }
        self.w.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}
