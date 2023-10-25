use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::ops::Add;

fn main() {
    read_lines("info.txt").expect("panic message");

    // let mut user_input = String::new();
    // io::stdin().read_line(&mut user_input).unwrap();
    // println!("Output: {}", convert(&user_input));
}

fn read_lines(args: &str) -> io::Result<()> {
    let file = File::open(args)?;
    let reader = BufReader::new(file);
    let mut string: String = String::new();

    for line in reader.lines() {
        string.push_str(convert(line.unwrap().to_string()).as_str());
        string.push_str("\n");
    }

    let mut create_file = File::create(args.to_string().add("_converted.txt"))?;
    create_file.write_all(string.as_ref())?;

    Ok(())
}

fn convert(input: String) -> String {
    let mut letters_map = HashMap::new();
    letters_map.insert("'", "");
    letters_map.insert("а", "a");
    letters_map.insert("б", "b");
    letters_map.insert("в", "w");
    letters_map.insert("г", "h");
    letters_map.insert("ґ", "g");
    letters_map.insert("д", "d");
    letters_map.insert("е", "e");
    letters_map.insert("є", "je");
    letters_map.insert("ж", "ż");
    letters_map.insert("з", "z");
    letters_map.insert("и", "y");
    letters_map.insert("і", "i");
    letters_map.insert("ї", "ji");
    letters_map.insert("й", "j");
    letters_map.insert("к", "k");
    letters_map.insert("л", "ł");
    letters_map.insert("м", "m");
    letters_map.insert("н", "n");
    letters_map.insert("о", "o");
    letters_map.insert("п", "p");
    letters_map.insert("р", "r");
    letters_map.insert("с", "s");
    letters_map.insert("т", "t");
    letters_map.insert("у", "u");
    letters_map.insert("ф", "f");
    letters_map.insert("х", "ch");
    letters_map.insert("ц", "c");
    letters_map.insert("ч", "cz");
    letters_map.insert("ш", "sz");
    letters_map.insert("щ", "szcz");
    letters_map.insert("ь", "");
    letters_map.insert("ю", "ju");
    letters_map.insert("я", "ja");

    let mut soft_letters = HashMap::new();
    soft_letters.insert("д", "ď");
    soft_letters.insert("з", "ź");
    soft_letters.insert("л", "l");
    soft_letters.insert("н", "ń");
    soft_letters.insert("р", "ŕ");
    soft_letters.insert("с", "ś");
    soft_letters.insert("ц", "ć");
    soft_letters.insert("т", "ť");

    let mut vec: Vec<char> = Vec::new();
    let string: &str = input.as_str();
    let mut output: String = String::new();

    for c in string.chars() {
        vec.push(c);
    }

    let vowels = ["а", "е", "і", "и", "є", "у", "о", "я", "ю", "ї"];
    let mut vowels_map = HashMap::new();
    vowels_map.insert("я", "ia");
    vowels_map.insert("є", "ie");
    vowels_map.insert("ю", "iu");
    let mut is_prev_consonant: bool = false;

    // Here we go
    for index in 0..vec.len() {
        let current_char = vec.get(index).unwrap();
        let lowered = current_char.to_string().to_lowercase();
        let mut next_letter = String::new();
        let mut next_letter_bool: bool = false;
        let mut is_soft: bool = false;

        if (index + 1) != vec.len() {
            let letter = &vec.get(index + 1).unwrap().clone();
            next_letter_bool = letter.is_uppercase();
            next_letter.push_str(letter.to_lowercase().to_string().as_str())
        }

        if letters_map.contains_key(lowered.as_str()) {
            let mut latin = letters_map.get(lowered.as_str()).unwrap();
            let is_upper = if current_char.is_uppercase() { true } else { false };

            if is_prev_consonant && vowels_map.contains_key(lowered.as_str()) {
                latin = vowels_map.get(lowered.as_str()).unwrap()
            }

            if next_letter == "ь" && soft_letters.contains_key(lowered.as_str()) {
                latin = soft_letters.get(lowered.as_str()).unwrap();
                is_soft = true;
            }

            // Change letter's case algorithm
            if is_upper {
                if latin.len() > 1 && !is_soft {
                    if next_letter_bool || !(letters_map.contains_key(next_letter.as_str())) {
                        output.push_str(latin.to_uppercase().as_str())
                    } else if !(next_letter_bool) || (next_letter != " ") {
                        output.push_str(to_title(latin).as_str())
                    } else { output.push_str(latin.to_uppercase().as_str()) }
                } else { output.push_str(latin.to_uppercase().as_str()) }
            } else { output.push_str(latin) }
        } else { output.push_str(&current_char.to_string()) }

        if letters_map.contains_key(&lowered.as_str()) {
            if !vowels.contains(&lowered.as_str()) {
                is_prev_consonant = true
            } else { is_prev_consonant = false }
        } else { is_prev_consonant = false }
    }
    output
}

fn to_title(string: &str) -> String {
    let first_letter = &string[0..1];
    let mut output: String = String::new();
    output.push_str(first_letter.to_uppercase().as_str());
    output.push_str(&string[1..string.len()]);
    output
}





























