use std::{collections::VecDeque, io::stdin};

fn main() {
    let input = stdin().lines();
    let mut cargo = "".to_string();
    let mut ships: Vec<VecDeque<char>> = vec![];

    for l in input {
        let line = l.unwrap().to_string();

        if line == "".to_string() {
            break;
        }

        cargo += &(cargo.clone() + "\r\n" + &line);

        for line in cargo.lines().skip(1) {
            for i in (0..line.len()).step_by(4) {}
        }
    }
}
