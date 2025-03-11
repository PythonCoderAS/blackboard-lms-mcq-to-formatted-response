// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::VecDeque;

use regex::Regex;

fn transform_text_to_formatted_string(
    text: &str,
    num_choices: u8,
    multiple_choices_allow: bool,
) -> String {
    let regex = Regex::new(r"[\r\n]{2,}").unwrap();
    let lines: Vec<String> = regex
        .replace_all(text, "\n")
        .trim()
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    let mut result_pieces: VecDeque<String> = VecDeque::with_capacity(lines.len());
    let mut counter: u8 = 0;
    assert!(
        num_choices <= 26,
        "Number of choices must be less than or equal to 26"
    );
    for line in lines.iter().rev() {
        if line.is_empty() {
            continue;
        }
        if counter > num_choices {
            result_pieces.push_front("".to_owned());
            result_pieces.push_front(line.clone());
        } else if counter == num_choices {
            result_pieces.push_front("".to_owned());
            if multiple_choices_allow {
                result_pieces.push_front("Multiple choices allowed.\n".to_owned());
            }
            result_pieces.push_front(line.clone());
        } else {
            result_pieces.push_front(format!(
                "{}. {}",
                (b'a' + num_choices - 1 - counter) as char,
                line
            ));
        }
        counter += 1;
    }
    result_pieces
        .into_iter()
        .collect::<Vec<String>>()
        .join("\n")
}

#[tauri::command]
fn tauri_transform_text_to_formatted_string(
    text: String,
    num_choices: u8,
    multiple_choices_allow: bool,
) -> String {
    transform_text_to_formatted_string(&text, num_choices, multiple_choices_allow)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tauri_transform_text_to_formatted_string
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
