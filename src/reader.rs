use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::io;
use std::ops::Add;

use crate::converter;

pub fn read_lines(args: &str) -> io::Result<()> {
    let file = File::open(args)?;
    let reader = BufReader::new(file);
    let mut string: String = String::new();

    for line in reader.lines() {
        string.push_str(converter::convert(line.unwrap().to_string()).as_str());
        string.push_str("\n");
    }

    let mut create_file = File::create(args.replace(".txt", "").to_string().add("_converted.txt"))?;
    create_file.write_all(string.as_ref())?;

    println!("Success!");
    Ok(())
}
