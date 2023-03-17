use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use crc::crc32::checksum_ieee;
use std::{fmt, str};
use std::path::Path;
use std::io::{BufReader, Read};
use std::fs::{File, read};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Png {
    chunks: Vec<Chunk>
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    // constructs Png from chunks
    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png{chunks}
    }

    // creates png struct from file path 
    // <P: AsRef<Path>> ensures that reference of generic P can be converted into std::path::Path
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let buffer = read(path).unwrap();
        let png = Png::try_from(buffer.as_ref()).unwrap();
        Ok(png)
    }

    pub fn header(&self) -> &[u8; 8] {
        &Png::STANDARD_HEADER
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    // removes first occurrence of specified chunk type
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk, String> {
        for (i, chunk) in self.chunks.iter().enumerate() {
            if *chunk.chunk_type().to_string() == chunk_type.to_string() {
                return Ok(self.chunks.remove(i));
            }
        }
        return Err(format!("Chunk {:?} not found", chunk_type));
    }

    // searches of first occurrence of specified chunk type and returns the chunk
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        for chunk in self.chunks.iter() {
            if *chunk.chunk_type().to_string() == chunk_type.to_string() {
                return Some(&chunk);
            }
        }
        return None;
    }

    // lists chunks contained in png
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    // returns byte representation of png file (includes header followed by all chunks)
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut head: Vec<u8> = Png::STANDARD_HEADER.to_vec();
        for chunk in self.chunks.iter() {
            head.append(&mut chunk.as_bytes());
        }
        return head;
    }
}

// constructs png from stream of u8
impl TryFrom<&[u8]> for Png {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> crate::Result<Self> {
        // case when only header is received as png
        if bytes.len() == 8 {
            if bytes != Png::STANDARD_HEADER {
                return Err("invalid png header".into());
            }
            return Ok(Png{chunks: vec![]});
        }
        // check validity of header
        let (header, mut rest) = bytes.split_at(8);
        if *header != Png::STANDARD_HEADER {
            return Err("invalid png header".into());
        }

        // get length of next chunk
        let (len, _) = rest.split_at(4);
        let mut len_u32 = u32::from_be_bytes(len.try_into().expect("Could not convert slice to array"));
        // initialize chunks and append them to png file
        // loop iterates until all bytes are exhausted
        let mut chunks: Vec<Chunk> = vec![];
        
        loop {
            let (b, new_rest) = rest.split_at((12 + len_u32) as usize); // 4 bytes for len, 4 for chunk type, 4 for crc
            rest = new_rest;
            let chunk = Chunk::try_from(b).unwrap();
            chunks.push(chunk);

            if rest.len() == 0 {
                break;
            }
            // re-calulate remaining byte length
            let (len, _) = rest.split_at(4);
            len_u32 = u32::from_be_bytes(len.try_into().expect("Could not convert slice to array"));
        }
        //chunks.push(Chunk::try_from(rest).unwrap());
        Ok(Png{chunks})
    }
}


impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Png {{",)?;
        writeln!(f, "  Chunks: {}", self.chunks().len())?;
        writeln!(f, "  Size: {}", self.as_bytes().len())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

