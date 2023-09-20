use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;
use clap::Parser;
use color_eyre::eyre::{Result, Context};


#[derive(Parser, Debug)]
struct Cat {

    files_path: Vec<std::path::PathBuf>
}

#[derive(Error, Debug)]
pub enum CatError {

    #[error("Could not open file {0}")]
    CouldNotOpenFile(PathBuf)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    color_eyre::install()?;
    let cat = Cat::parse();

    for file_path in cat.files_path {
        let file = File::open(&file_path).with_context(|| CatError::CouldNotOpenFile(file_path))?;
        let mut buf = String::new();
        let mut reader = BufReader::new(file);
        
        loop {
            let data_size = match reader.read_line(&mut buf) {
                Ok(size) => size,
                Err(e) => panic!("Error reading file {:?}", e)
            };
            if data_size == 0 {
                break;
            };
            print!("{}", buf);
            buf.clear();
        }
    }
    Ok(())

}