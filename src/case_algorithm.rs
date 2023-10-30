use std::collections::HashMap;

fn to_title(string: String) -> String {
    let mut output: String = String::new();
    let first_letter = &string[0..1];
    output.push_str(first_letter.to_uppercase().as_str());
    output.push_str(&string[1..string.len()]);
    output
}

pub fn change_case(letters_map: &HashMap<&str, &str>, latin: String,
                   next_letter: String, is_upper: bool, is_soft: bool,
                   next_letter_bool: bool) -> String {
    if is_upper {
        if latin.len() > 1 && !is_soft && (latin != "ł".to_string()) && (latin != "ż".to_string())
        {
            if next_letter_bool || !(letters_map.contains_key(next_letter.as_str())) {
                latin.to_uppercase()
            } else if !(next_letter_bool) {
                to_title(latin)
            } else { latin.to_uppercase() }
        } else { latin.to_uppercase() }
    } else { latin.to_string() }
}