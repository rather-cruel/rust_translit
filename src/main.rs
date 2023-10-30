use std::io::{prelude::*};
use std::io;
use std::process::Command;

mod reader;
mod converter;
mod case_algorithm;

fn main() {
    let mut user_input = String::new();
    print!("Введіть адресу вашого файлу [Приклад: C:\\Users\\me\\Desktop\\file.txt]: ");
    io::stdout().flush().expect("flush failed.");
    io::stdin().read_line(&mut user_input).unwrap();
    reader::read_lines(user_input.trim()).expect("input error");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
