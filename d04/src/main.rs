use std::fs;

fn get_range(range: Vec<&str>) -> (usize, usize) {
    let f: usize = range[0].parse().expect("NaN");
    let s: usize = range[1].parse().expect("NaN");

    (f, s)
}

fn first_half(content: &str) -> usize {
    let lines = content.split('\n');
    let mut sum: usize = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let pairs: Vec<&str> = line.split(',').collect();

        let first = pairs[0];
        let second = pairs[1];

        let (ff, fs) = get_range(first.split('-').collect());
        let (sf, ss) = get_range(second.split('-').collect());

        if (ff >= sf && fs <= ss) || (sf >= ff && ss <= fs) {
            sum = sum + 1;
        }
    }

    sum
}

fn second_half(content: &str) -> usize {
    let lines = content.split('\n');
    let mut sum: usize = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let pairs: Vec<&str> = line.split(',').collect();

        let first = pairs[0];
        let second = pairs[1];

        let (ff, fs) = get_range(first.split('-').collect());
        let (sf, ss) = get_range(second.split('-').collect());

        if (ff >= sf && ff <= ss)
            || (fs >= sf && fs <= ss)
            || (sf >= ff && sf <= fs)
            || (ss >= ff && ss <= fs)
        {
            sum = sum + 1;
        }
    }

    sum
}

fn main() {
    const FILE_PATH: &str = "./assets/input.txt";

    let content = fs::read_to_string(FILE_PATH).expect("Could not open file");

    println!("{}", first_half(&content));
    println!("{}", second_half(&content));
}
