use std::io::stdin;

fn main() {
    let input = stdin().lines();
    let mut count = 0;

    for l in input {
        let line = l.unwrap().to_string();

        let parts: Vec<&str> = line.split(",").collect();

        let mut range: Vec<u32> = parts[0]
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let a: Vec<u32> = (range[0]..range[1] + 1).collect();

        range = parts[1]
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let b: Vec<u32> = (range[0]..range[1] + 1).collect();

        if a.contains(&b[0]) && a.contains(&b[b.len() - 1])
            || b.contains(&a[0]) && b.contains(&a[a.len() - 1])
        {
            count += 1;
        }
    }

    println!("Count: {}", count);
}
