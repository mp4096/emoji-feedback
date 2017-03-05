use std::path::Path;
use std::io::Error;


pub fn append_line_to_file<T, U>(path: T, data: U) -> Result<(), Error>
    where T: AsRef<Path>,
          U: AsRef<str>
{
    use std::fs::OpenOptions;
    use std::io::{BufWriter, Write};

    let file = OpenOptions::new().write(true)
        .append(true)
        .create(true)
        .open(path.as_ref())?;

    let mut buf = BufWriter::new(file);
    buf.write_all(data.as_ref().as_bytes())?;
    buf.write_all(b"\n")?;

    Ok(())
}
