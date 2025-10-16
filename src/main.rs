use rand::{prelude::IndexedRandom, rng};
use std::io;

fn main() {
    let hiragana = [
        "あ", "い", "う", "え", "お",
        "か", "き", "く", "け", "こ",
        "さ", "し", "す", "せ", "そ",
        "た", "ち", "つ", "て", "と",
        "な", "に", "ぬ", "ね", "の",
        "は", "ひ", "ふ", "へ", "ほ",
        "ま", "み", "む", "め", "も",
        "や", "ゆ", "よ",
        "ら", "り", "る", "れ", "ろ",
        "わ", "を",
        "ん",
        "が", "ぎ", "ぐ", "げ", "ご", // K → G
        "ざ", "じ", "ず", "ぜ", "ぞ", // S → Z
        "だ", "ぢ", "づ", "で", "ど", // T → D
        "ば", "び", "ぶ", "べ", "ぼ", // H → B
        "ぱ", "ぴ", "ぷ", "ぺ", "ぽ",
];
    let mut r = rng();

    loop {
        if let Some(&ch) = hiragana.choose(&mut r) {
            println!("What is this hiragana? {ch}");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.trim() == "break" {break};
            if input.trim() == ch {
                println!("Correct!");
            } else {
                println!("Wrong!");
            }
        } else {
            println!("No phrases available.");
        }
    }
}
