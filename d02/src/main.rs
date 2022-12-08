use std::{fs, collections::HashMap};

fn first_half(content: &str) -> usize {
    let lines = content.split("\n");

     /*
        rock = 1 pts
        paper = 2 pts
        scissors = 3 pts

        lost = 0
        draw = 3
        win = 6

            X   Y   Z
        A   3   6   0

        B   0   3   6

        C   6   0   3
      */

    let mut score: usize = 0;

    let battle_point: [[usize; 3]; 3] = [
        [3, 6, 0],
        [0, 3, 6],
        [6, 0, 3]
    ];

    let mut play_point: HashMap<&str, usize> = HashMap::new();
    play_point.insert("X", 1);
    play_point.insert("Y", 2);
    play_point.insert("Z", 3);

    let mut mapping_battle: HashMap<&str, usize> = HashMap::new();
    mapping_battle.insert("A", 0);
    mapping_battle.insert("B", 1);
    mapping_battle.insert("C", 2);

    mapping_battle.insert("X", 0);
    mapping_battle.insert("Y", 1);
    mapping_battle.insert("Z", 2);

    for line in lines {
        let mut round = line.split(" ");

        let left: &str = round.next().expect("Failed to get char");
        let right: &str = round.next().expect("Failed to get char");

        let play_point: &usize = play_point.get(right).expect("Failed to get value");

        let row_index: &usize = mapping_battle.get(left).expect("Failed to get value");
        let col_index: &usize = mapping_battle.get(right).expect("Failed to get value");

        score = score + battle_point[* row_index][* col_index] + play_point;
    }

    score
}

fn main() {
    const FILE_PATH: &str = "./assets/input.txt";

    let content = fs::read_to_string(FILE_PATH).expect("Could not open file");

    println!("{}", first_half(&content));
}
