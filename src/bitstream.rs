use std::{
    fmt,
    fs::File,
    io::Read,
    ops::{AddAssign, Range},
    path::Path
};

use miette::{IntoDiagnostic, Result};

use crate::{error::IceError, CHUNK_SIZE};

#[derive(Debug)]
pub struct Bitstream<'file> {
    pub filepath: &'file Path,
    pub buffer: Vec<u8>,
    pub filesize: usize,
    pub offset: usize
}

#[derive(Default)]
pub struct BitstreamOptions {
    pub offset: usize
}

impl<'file> Bitstream<'file> {
    pub fn new(
        filename: &'file str, options: BitstreamOptions
    ) -> Result<Self> {
        let filepath = Path::new(filename);
        if !filepath.exists() {
            return Err(IceError::InvalidFile(String::from(filename)).into())
        }
        let mut file = File::open(filename).into_diagnostic()?;
        let filesize = file.metadata().into_diagnostic()?.len() as usize;
        let mut buffer = Vec::with_capacity(filesize);
        file.read_to_end(&mut buffer).into_diagnostic()?;
        let BitstreamOptions { offset } = options;

        Ok(Self { filepath, buffer, filesize, offset })
    }

    pub fn chunk_iter(&self) -> ChunkIter {
        ChunkIter::new(self)
    }
}

impl<'bitstream> fmt::Display for Bitstream<'bitstream> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, bitstream size = {}",
            self.filepath.file_name().unwrap().to_string_lossy(),
            self.filesize
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Chunk(pub usize, pub Range<usize>, pub bool);

pub struct ChunkIter<'bitstream> {
    addr: usize,
    bitstream: &'bitstream Bitstream<'bitstream>
}

impl<'bitstream> ChunkIter<'bitstream> {
    pub fn new(bitstream: &'bitstream Bitstream) -> Self {
        ChunkIter { addr: bitstream.offset, bitstream }
    }
}

impl<'bitstream> Iterator for ChunkIter<'bitstream> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.addr >= self.bitstream.filesize {
            return None
        }

        let chunk = if self.addr + CHUNK_SIZE < self.bitstream.filesize {
            Chunk(self.addr, self.addr..self.addr + CHUNK_SIZE, false)
        } else {
            Chunk(self.addr, self.addr..self.bitstream.filesize, true)
        };
        self.addr.add_assign(CHUNK_SIZE);

        Some(chunk)
    }
}

#[cfg(test)]
#[path = "bitstream_test.rs"]
mod bitstream_test;
