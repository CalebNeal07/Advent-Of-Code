use std::io;

fn main() {
    let mut elf = 0;
    let input = io::stdin().lines();
    let mut elves: [u64; 3] = [0; 3];

    for line in input {
        let l = line.unwrap();

        if l == "" {
            for i in 0..3 {
                if elf > elves[i] {
                    for j in 0..3 {
                        if elves[i] > elves[j] {
                            for h in 0..3 {
                                if elves[j] > elves[h] {
                                    elves[h] = elves[j];
                                    break;
                                }
                            }

                            elves[j] = elves[i];
                            break;
                        }
                    }

                    elves[i] = elf;
                    break;
                }
            }

            elf = 0;
        } else {
            elf += l.parse::<u64>().unwrap();
        }
    }

    println!("{}, {}, {}", elves[0], elves[1], elves[2]);
    println!("Sum is {}", elves[0] + elves[1] + elves[2]);
}
