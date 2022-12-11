use std::{collections::HashMap, fs};

fn get_letter_value(letter: char) -> usize {
    let letter_mapping: HashMap<char, usize> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    return letter_mapping[&letter];
}

fn first_half(content: &str) -> usize {
    let lines = content.split('\n');
    let mut letters: Vec<char> = Vec::new();
    let mut first_compartment_letters: HashMap<char, bool> = HashMap::new();
    let mut sum: usize = 0;

    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);

        for letter in first.chars() {
            first_compartment_letters.insert(letter, true);
        }

        for letter in second.chars() {
            if first_compartment_letters.contains_key(&letter) {
                letters.push(letter);
                break;
            }
        }

        first_compartment_letters.clear();
    }

    for letter in letters {
        sum = sum + get_letter_value(letter);
    }

    sum
}

// fn second_half(content: &str) -> usize {
//     let lines = content.split('\n');
// }

fn main() {
    const FILE_PATH: &str = "./assets/input.txt";

    let content = fs::read_to_string(FILE_PATH).expect("Could not open file");

    println!("{}", first_half(&content));
    // println!("{}", second_half(&content));
}
