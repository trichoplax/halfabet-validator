use wasm_bindgen::prelude::*;
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[wasm_bindgen]
pub fn halfabet_validation(text: &str) -> String {
    let letters = letters_present(text);
    let letter_count = letters.len();
    let absent = letters_absent(text);
    let absent_count = absent.len();
    format!(
        "{} distinct letters.<br>{} letters present: {}<br>{} letters absent: {}",
        match letter_count {
            x if x < 13 => format!("{} too few", 13 - x),
            x if x > 13 => format!("{} too many", x - 13),
            _ => "Exactly 13".to_string(),
        },
        letter_count,
        letters,
        absent_count,
        absent
    )
}

fn letters_present(text: &str) -> String {
    ALPHABET
        .chars()
        .filter(|&letter| {
            text.to_ascii_lowercase()
                .chars()
                .any(|character| character == letter)
        })
        .collect()
}

fn letters_absent(text: &str) -> String {
    ALPHABET
        .chars()
        .filter(|&letter| {
            !text
                .to_ascii_lowercase()
                .chars()
                .any(|character| character == letter)
        })
        .collect()
}
