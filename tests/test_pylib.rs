extern crate pylib;
extern crate tempfile;
use anyhow::Result;
use pylib::FileReadBytes;
use pylib::FileReadString;
use pylib::FileWriteBytes;
use pylib::FileWriteString;
use pylib::StdinReadLine;
use std::fs::OpenOptions;

use tempfile::NamedTempFile;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tmp_file() -> Result<()> {
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
    fn test_read_stdin() -> Result<()> {
        let line = std::io::stdin().read_string()?;
        println!("stdin: {}", line);
        Ok(())
    }

    #[test]
    fn test_time() -> Result<()> {
        let t1 = pylib::time();
        let t2 = pylib::time();
        assert!(t2 > t1);
        Ok(())
    }

    #[test]
    fn test_random() -> Result<()> {
        let t1 = pylib::time();
        let _rng = pylib::random::reseed_from_f64(t1);
        println!("{}", pylib::random::random());
        println!("{}", pylib::random::random());
        Ok(())
    }

    #[test]
    fn test_read_write_bytes() -> Result<()> {
        let temp_file: _ = NamedTempFile::new()?;
        let file_path: _ = temp_file.path();
        {
            let mut f: _ = OpenOptions::new().write(true).open(file_path)?;
            let s = String::from("hello");
            f.write_bytes(&s.into_bytes())?
        }
        {
            let mut f: _ = OpenOptions::new().read(true).open(file_path)?;
            let out = f.read_bytes(3)?;
            let expected = String::from("hel").into_bytes();
            assert_eq!(out, expected);
            let out2 = f.read_bytes(2)?;
            let expected2 = String::from("lo").into_bytes();
            assert_eq!(out2, expected2);
        }
        Ok(())
    }
}
