use std::fs;

fn first_half(content: &str) -> u32 {
    let lines = content.split("\n");

    let mut sum: u32 = 0;
    let mut highest: u32 = 0;

    for line in lines {
        if line.is_empty() {
            if sum > highest {
                highest = sum;
            }

            sum = 0;
            continue;
        }

        let line: u32 = line.trim().parse().expect("NaN");
        sum = sum + line;
    }

    highest
}

fn second_half(content: &str) -> u32 {
    let lines = content.split("\n");

    let mut elves_calories: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for line in lines {
        if line.is_empty() {
            elves_calories.push(sum);
            sum = 0;
            continue;
        }

        let line: u32 = line.trim().parse().expect("NaN");
        sum = sum + line;
    }

    sum = 0;
    elves_calories.sort_by(|a, b| b.cmp(a));

    for i in 0..3 {
        sum = sum + &elves_calories[i];
    }

    sum
}

fn main() {
    const FILE_PATH: &str = "./assets/input.txt";

    let content = fs::read_to_string(FILE_PATH).expect("Could not open file");

    println!("{}", first_half(&content));
    println!("{}", second_half(&content));
}
