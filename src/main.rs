//mod args;
mod tests;
mod chunk;
mod chunk_type;
//mod commands;
mod png;

extern crate crc;

use std::str::FromStr;
use std::convert::TryFrom;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// structopt crate for parsing command line arguments
#[derive(Debug, StructOpt)]
#[structopt(name="pngme", about="encodes message in png files.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(short="l", long="list")]
    list: bool,

    #[structopt(required_unless("list"), conflicts_with("list"))]
    chunk_type: Option<String>,

    #[structopt(short="d", long="decode", conflicts_with("encode"))]
    decode: bool,

    #[structopt(short="r", long="remove", conflicts_with("encode"))]
    remove: bool,

    #[structopt(short="e", long="encode", conflicts_with("decode"), conflicts_with("list"))]
    encode: Option<String>,
}

// encodes secret message into png file
fn encode_message(path: &PathBuf, chunk_type: &str, message: &str) -> Result<()> {
    let mut png = png::Png::from_file(path).unwrap();
    let chunk = chunk::Chunk::new(chunk_type.to_string(), message.as_bytes().to_vec());
    png.append_chunk(chunk);
    // save updated png file
    let png_bytes = png.as_bytes();
    let mut file = OpenOptions::new().read(true).write(true).create(false).open(path)?;
    file.write_all(&png_bytes).expect("Write to file failed");
    Ok(())
}

// reads secret message in png file, returns it as string
fn decode_message(path: &PathBuf, chunk_type: &str) -> Result<String> {
    let png = png::Png::from_file(path).unwrap();
    let chunk = png.chunk_by_type(chunk_type);
    if chunk.is_none() {
        Err(format!("Could not find chunk type `{}`", chunk_type).into())
    } else {
        Ok(chunk.unwrap().data_as_string().unwrap())
    }
}

// finds first occurence of specified chunk type and deletes said chunk, saves png file after deletion
fn remove_message(path: &PathBuf, chunk_type: &str) -> Result<()> {
    let mut png = png::Png::from_file(path).unwrap();
    let chunk = png.remove_chunk(chunk_type);
    if chunk.is_err() {
        Err(format!("Could not find chunk type `{}`", chunk_type).into())
    } else {
        // save updated png file
        let png_bytes = png.as_bytes();
        let mut file = std::fs::File::create(path)?;
        file.write_all(&png_bytes).expect("Write to file failed");
        Ok(())
    }
}

// lists all chunk types of png file and their size in bytes
fn list_png_info(path: &PathBuf) {
    let png = png::Png::from_file(path).unwrap();
    for c in png.chunks() {
        println!("chunk type: {}, {} bytes", c.chunk_type(), c.as_bytes().len());
    }
}

fn main() -> Result<()> { 
    let opt = Opt::from_args();
    if opt.list {
        list_png_info(&opt.path);
            Ok(())
    } else {
        let chunk_t = opt.chunk_type.unwrap();

        // check validity of passed chunk type 
        if !chunk_type::is_valid(&chunk_t) {
            Err(format!("Invalid chunk type `{}`", chunk_t).into())
        } else {
            if opt.decode {
                println!("{}", decode_message(&opt.path, &chunk_t)?);
                Ok(())
            } else if opt.list {
                list_png_info(&opt.path);
                Ok(())
            } else if opt.remove {
                remove_message(&opt.path, &chunk_t)
            } else {
                encode_message(&opt.path, &chunk_t, &(opt.encode.unwrap()));
                Ok(())
            }
        }
    }
}