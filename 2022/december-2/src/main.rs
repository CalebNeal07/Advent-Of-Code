use std::io;

fn won(a: char, b: char) -> u64 {
    /*
     * 'X' means lose
     * 'Y' means draw
     * 'Z' means win
     *
     * 1 Point for rock, 2 for paper, and 3 for scissors.
     */
    return match a {
        'A' => match b {
            // Rock
            'X' => 0 + 3,
            'Y' => 3 + 1,
            'Z' => 6 + 2,
            _ => 0,
        },
        'B' => match b {
            // Paper
            'X' => 0 + 1,
            'Y' => 3 + 2,
            'Z' => 6 + 3,
            _ => 0,
        },
        'C' => match b {
            // Scissors
            'X' => 0 + 2,
            'Y' => 3 + 3,
            'Z' => 6 + 1,
            _ => 0,
        },
        _ => 0,
    };
}

fn main() {
    let input = io::stdin().lines();
    let mut total: u64 = 0;

    for l in input {
        let line = l.unwrap();

        total += won(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
    }

    println!("Total Score: {}", total);
}
