use chrono::Local;
use tsss::TS;

// TODO
// * add arguments? picoargs?

fn main() -> std::io::Result<()> {
    let start = Local::now();
    let stdout_handle = std::io::stdout().lock();
    let mut stdin_handle = std::io::stdin().lock();
    std::io::copy(
        &mut stdin_handle,
        &mut TS::new(start, stdout_handle),
    )?;

    Ok(())
}