extern crate pylib;
extern crate tempfile;
use pylib::FileReadString;
use pylib::FileWriteString;
use std::fs::OpenOptions;

use tempfile::NamedTempFile;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tmp_file() -> Result<(), std::io::Error> {
        let temp_file: _ = NamedTempFile::new()?;
        let file_path: _ = temp_file.path();
        {
            let mut f: _ = OpenOptions::new().write(true).open(file_path)?;
            f.write_string("hello")?;
        }
        {
            let mut f: _ = OpenOptions::new().read(true).open(file_path)?;
            println!("{}", f.read_string()?);
        }
        Ok(())
    }
}

