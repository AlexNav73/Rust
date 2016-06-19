
use std::fs::File as FsFile;
use std::io::{Write, Error};
use super::Protocol;

pub struct File(Vec<u8>);

use std::path::Path;

impl File {
    
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File, Error> {
        let mut file = try!(FsFile::open(path));
        let mut buf = Vec::new();
        
        use std::io::Read;
        try!(file.read_to_end(&mut buf));

        Ok(File(buf))
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = try!(FsFile::create(path));
        file.write_all(&self.0)
    }

}

impl From<Vec<Protocol>> for File {
    fn from(mut ps: Vec<Protocol>) -> File {
        let mut res = Vec::new();
        
        for prot in &mut ps {
            res.append(match *prot {
                Protocol::FileBatch { ref mut data, .. } => data,
                _ => unreachable!()
            });
        }
        File(res)
    }
}
