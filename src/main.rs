use std::collections::HashSet;

fn main() {
    println!("How many pitches");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    println!("What base frequency (default: 1)");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut base_freq = 1.;
    match input.trim().parse::<f64>() {
        Ok(i) => base_freq = i,
        Err(_) => {},
    }

    let mut ratios = HashSet::new();
    let mut x = 1.;
    while n > ratios.len() {
        let mut i = x;
        loop {
            if i >= 2. {
                i /= 2.;
            } else {
                ratios.insert(i.to_string());
                break;
            }
        }
        x += 1.;
    }
    let mut numbers = Vec::new();
    for i in ratios.iter() {
        numbers.push(i);
    }
    numbers.sort();
    for i in numbers {
        println!("{}", i.parse::<f64>().unwrap() * base_freq);
    }
}
