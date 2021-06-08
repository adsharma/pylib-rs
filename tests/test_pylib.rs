extern crate pylib;
extern crate tempfile;
use pylib::FileReadString;
use pylib::FileWriteString;
use pylib::StdinReadLine;
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

    #[test]
    #[ignore] // Disabled because it involves reading stdin
    fn test_read_stdin() -> Result<(), std::io::Error> {
        let line = std::io::stdin().read_string()?;
        println!("stdin: {}", line);
        Ok(())
    }

    #[test]
    fn test_time() -> Result<(), std::io::Error> {
        let t1 = pylib::time();
        let t2 = pylib::time();
        assert!(t2 > t1);
        Ok(())
    }

    #[test]
    fn test_random() -> Result<(), std::io::Error> {
        let t1 = pylib::time();
        let _rng = pylib::random::reseed_from_f64(t1);
        println!("{}", pylib::random::random());
        println!("{}", pylib::random::random());
        Ok(())
    }
}
