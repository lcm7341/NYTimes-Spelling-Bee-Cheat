use std::io::{self, Read};
use xz2::read::XzDecoder;

const WORDS_XZ: &[u8] = include_bytes!("words.txt.xz");

fn main() {
    // Decompress the data
    let mut decoder = XzDecoder::new(WORDS_XZ);
    let mut uncompressed_data = String::new();
    decoder.read_to_string(&mut uncompressed_data).expect("Failed to decompress data");

    // Process the uncompressed data
    let words_txt = uncompressed_data.replace("\r", "");
    let words: Vec<&str> = words_txt.split('\n').collect();

    let mut input = String::new();

    println!("Enter the YELLOW letter:");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let must_include = input.trim().chars().next().unwrap();

    input.clear();
    println!("Enter the GREY letters:");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let use_letters = input.trim().to_string() + must_include.to_string().as_str();

    println!("\nWords:\n");

    let filtered_words = words
        .iter()
        .filter(|&word| {
            word.chars().all(|c| use_letters.contains(c))
                && word.contains(must_include)
                && word.len() >= 4
        })
        .collect::<Vec<_>>();

    for word in filtered_words.iter() {
        println!("{}", word);
    }
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Couldnt read dat shit");
}
