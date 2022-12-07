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

    return highest;
}

fn second_half(content: &str) -> u32 {
    // @todo
    return 0;
}

fn main() {
    const FILE_PATH: &str = "./assets/input.txt";

    let content = fs::read_to_string(FILE_PATH).expect("Could not open file");

    println!("{}", first_half(&content));
}
