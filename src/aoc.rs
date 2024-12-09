use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, BufRead, Read, Lines};

pub struct FileCharIterator {
    reader: BufReader<File>
}

impl FileCharIterator {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(FileCharIterator { reader })
    }
}

impl Iterator for FileCharIterator {
    type Item = io::Result<char>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(Ok(buf[0] as char)),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn read_chars<P>(path: P) -> io::Result<FileCharIterator>
where
 P: AsRef<Path>
{
    FileCharIterator::new(path)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


