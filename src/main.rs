extern crate core;

mod leetcode;
mod exercism_rust;
mod high_scores;
mod build_proverb;
mod bob_reply;
mod grains;
mod nth_prime;
mod raindrops;
mod collatz;
mod series;
mod diffie_hellman;
mod armstrong;
mod acronym;
mod luhn_checksum;
mod factors;
mod roster;
mod yacht;
mod plants;
mod learn_macro;
mod isbn_verifier;
mod hamming;
mod isogram;
mod rle;
mod bowling;
mod etl;
mod phone_number;
mod roman;
mod sum_of_multiples;
mod palindrome_products;
mod pascals_triangle;
mod perfect_numbers;
mod primes_up_to;
mod pangram;
mod robot_simulator;
mod say_english;
mod scrabble_score;
mod nucleotide_count;
mod missing_repeated_values;
mod simple_cipher;
mod anagram;
mod luhn_from;
mod luhn_trait;
mod atbash_cipher;
mod triangle;
mod rna_transcription;
mod accumulate;
mod knapsack;
mod spiral_matrix;
mod binary_search;
mod diamond;
mod pythagorean_triplet;
mod crypto_square;
mod largest_series_product;
mod rail_fence;
mod design_sql;
mod gigasecond;
mod clock;
mod triangle_type;
mod brackets;
mod custom_set;
mod all_your_base;
mod alphametics;
mod space_age;
mod sub_list;
mod secret_handshake;
mod rotational_cipher;
mod grep;
mod book_store;
mod tally;
mod word_count;
mod allergies;
mod leetcode2;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("Hello, world!");
    let word = first_word(&my_string);
    println!("first word is: {}", word);
}