use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Debug)]
pub struct ChunkType {
    signature: String
}

// checks that string passed as chunk_type is valid
pub fn is_valid(s: &str) -> bool {
    // string should be 4-bytes long
    if s.len() != 4 {
        return false;
    }      
    // string should contain characters (A-Z, a-z, 65-90, 97-122)
    for c in s.chars() {
        if !char::is_digit(c, 36) || char::is_numeric(c) {
            return false;
        }
    }
    true
}

// Valid bytes are represented by the characters A-Z or a-z
pub fn is_valid_byte(byte: u8) -> bool {
    let b = byte as char;
    if !b.is_ascii_alphanumeric() {
        false
    } else {
        true
    }
}

impl ChunkType {
    // returns byte array representation of type string
    pub fn bytes(&self) -> [u8; 4] {
        let mut ar: [u8; 4] = [0; 4];
        for i in 0..4 {
            ar[i] = self.signature.as_bytes()[i];
        }
        ar
    }

    pub fn to_string(&self) -> String {
        String::from(&self.signature)
    }
    
    // Ancillary bit: bit 5 of first byte (0=critical, 1=ancillary)
    pub fn is_critical(&self) -> bool {
        let byte = self.signature.as_bytes()[0];
        byte.is_ascii_uppercase()
    }
    // Private bit: bit 5 of second byte (0=public, 1=private)
    pub fn is_public(&self) -> bool {
        let byte = self.signature.as_bytes()[1];
        byte.is_ascii_uppercase()
    }
    //Reserved bit: bit 5 of third byte (Must be 0)
    pub fn is_reserved_bit_valid(&self) -> bool {
        let byte = self.signature.as_bytes()[2];
        byte.is_ascii_lowercase()
    }
    // Safe-to-copy bit: bit 5 of fourth byte (0=unsafe to copy, 1=safe to copy)
    pub fn is_safe_to_copy(&self) -> bool {
        let byte = self.signature.as_bytes()[3];
        byte.is_ascii_lowercase()
    }

    pub fn is_valid(&self) -> bool {
        is_valid(&self.signature)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.signature)
    }
}

// constructs object from byte array
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        Ok(ChunkType{signature: std::str::from_utf8(&bytes).unwrap().to_string()})
    }
}

// constructs object from string
impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(ChunkType{signature: s.to_string()})
    }
}
