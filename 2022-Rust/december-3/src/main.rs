use std::io;

fn main() {
    let input = io::stdin().lines();
    let mut tokens: Vec<char> = vec![];
    let mut count: u8 = 0;
    let mut data: [String; 2] = ["".to_string(), "".to_string()];

    for l in input {
        let line = l.unwrap();

        match count {
            0 | 1 => {
                count += 1;
                if data[0] == "" {
                    data[0] = line.to_string();
                } else {
                    data[1] = line.to_owned();
                }
            }
            2 => {
                for c in line.chars() {
                    if data[0].contains(c) && data[1].contains(c) {
                        tokens.append(&mut vec![c]);
                        break;
                    }
                }

                count = 0;
                data = ["".to_string(), "".to_string()];
            }
            _ => {
                println!("Something wierd happened")
            }
        }
    }

    let mut total: u64 = 0;

    for token in tokens {
        if token as u8 >= 97 {
            total += (token as u8 - 96) as u64;
        } else {
            total += (token as u8 - 38) as u64;
        }
    }

    println!("Total:\t{}", total);
}
