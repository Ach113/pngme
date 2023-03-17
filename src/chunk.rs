use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use crc::crc32::checksum_ieee;
use std::{fmt, str, error::Error};
//use crate::Result;
use crc::crc32;

/* NOTES: 
    from_be_bytes -> converts byte array (big endian notation) into respective primitive type
    try_into() tries to convert type to required datatype (or at least tries to)
*/

use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_t: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

// error enum for chunk struct
#[derive(Debug)]
pub enum ChunkError {
    CRC,
    Other
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChunkError::CRC => write!(f, "CRC check failed!"),
            ChunkError::Other => write!(f, "Unknown error!"),
        }
    }
}

// Allow this type to be treated like an error
impl Error for ChunkError {
    fn description(&self) -> &str {
        match *self {
            ChunkError::CRC => "CRC check failed!",
            ChunkError::Other => "Unknown error!",
        }
    }
}

impl Chunk {
    // constructor
    pub fn new(chunk_t: String, chunk_data: Vec<u8>) -> Chunk {
        let chunk_t = ChunkType::from_str(&chunk_t).unwrap();
        let len: u32 = chunk_data.len().try_into().unwrap();
        let crc_bytes: Vec<u8> = chunk_t.bytes().iter()
        .chain(chunk_data.iter())
        .copied().collect();
        let crc = checksum_ieee(&crc_bytes);
        Chunk{length: len, chunk_t: chunk_t, chunk_data: chunk_data.to_vec(), crc: crc}
    }

    // getters
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_t
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn data_as_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(str::from_utf8(&self.chunk_data).unwrap().to_string())
    }

    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self.chunk_t.bytes().iter()
        .chain(self.chunk_data.iter())
        .copied().collect();
        checksum_ieee(&bytes)
    }

    // returns whole chunk as a sequence of bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = 
        self.length.to_be_bytes().iter()
        .chain(self.chunk_t.bytes().iter()
        .chain(self.chunk_data.iter()
        .chain(self.crc.to_be_bytes().iter())))
        .copied().collect();
        return bytes;
    }
}

// constructs chunk struct from u8 array
impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChunkError> {
        let (length, rest) = bytes.split_at(4);
        let length = u32::from_be_bytes(length.try_into().expect("Could not convert slice to array"));

        let (chunk_str, rest) = rest.split_at(4);
        let chunk_str = str::from_utf8(chunk_str).unwrap();
        let chunk_t = ChunkType::from_str(&chunk_str).unwrap();

        let (chunk_data, crc) = rest.split_at(length as usize);
        let chunk_data: Vec<u8> = chunk_data.to_vec();

        let crc = u32::from_be_bytes(crc.try_into().expect("Could not convert slice to array"));
        // construct chunk
        let chunk = Chunk{length, chunk_t, chunk_data: chunk_data, crc};
        // check validity of crc
        if crc == chunk.crc() {
            Ok(chunk)
        } else {
            Err(ChunkError::CRC)
        }
    }
}


impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
