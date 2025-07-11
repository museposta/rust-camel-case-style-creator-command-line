use std::io::{self, BufRead};

fn process_line(line: &str) -> String {
    let parts: Vec<&str> = line.split(';').collect();
    if parts.len() != 3 {
        return String::new();
    }

    let operation = parts[0];
    let name_type = parts[1];
    let words = parts[2];

    match operation {
        "S" => split(words, name_type),
        "C" => combine(words, name_type),
        _ => String::new(),
    }
}

fn split(input: &str, name_type: &str) -> String {
    let mut result = String::new();
    let mut input = input.to_string();

    // Remove parentheses for methods
    if name_type == "M" {
        input = input.replace("()", "");
    }

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            result.push(' ');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result.trim().to_string()
}

fn combine(input: &str, name_type: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    // Handle first word
    match name_type {
        "C" => {
            // Class name - capitalize first letter of first word
            let mut first_word = words[0].chars();
            if let Some(c) = first_word.next() {
                result.push(c.to_ascii_uppercase());
                result.extend(first_word);
            }
        }
        _ => {
            // Method or variable - keep first word lowercase
            result.push_str(words[0]);
        }
    }

    // Handle remaining words
    for word in words.iter().skip(1) {
        let mut chars = word.chars();
        if let Some(c) = chars.next() {
            result.push(c.to_ascii_uppercase());
            result.extend(chars);
        }
    }

    // Add parentheses for methods
    if name_type == "M" {
        result.push_str("()");
    }

    result
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        println!("{}", process_line(&line));
    }
}