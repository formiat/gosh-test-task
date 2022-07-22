use crate::config::Config;
use crate::merge::compressive_merge;

mod config;
mod merge;

fn main() {
    let config = Config::new();

    let input_length: usize = config.words.iter().map(|v| v.chars().count()).sum();
    println!("Input words character count: {}", input_length);

    let res_str = compressive_merge(config.words);
    let output_length = res_str.chars().count();
    let compression_rate = 1.0 - (output_length as f64 / input_length as f64);
    let compression_rate = format!("{:.0}", compression_rate * 100.0);
    println!("Answer: {}", res_str);
    println!("Output string character count: {}", output_length);
    println!("Compression rate (compression %): {}%", compression_rate);
}
