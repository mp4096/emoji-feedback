use std::io::Error;
use std::path::Path;

pub fn append_line_to_file<T, U>(path: T, data: U) -> Result<(), Error>
where
    T: AsRef<Path>,
    U: AsRef<str>,
{
    use std::fs::OpenOptions;
    use std::io::{BufWriter, Write};

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path.as_ref())?;

    let mut buf = BufWriter::new(file);
    buf.write_all(data.as_ref().as_bytes())?;
    buf.write_all(b"\n")?;

    Ok(())
}

pub fn read_file<T>(path: T) -> Result<String, Error>
where
    T: AsRef<Path>,
{
    use std::fs::File;
    use std::io::{BufReader, Read};

    let file = File::open(path.as_ref())?;
    let mut buf = BufReader::new(file);
    let mut content = String::new();

    buf.read_to_string(&mut content)?;

    Ok(content)
}
