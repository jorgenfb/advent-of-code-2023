fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}\n", process(input));
}

fn process(input: &str) -> u64 {
    let mut data = input.lines().map(|line| {
        line.split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    });

    let times = data.next().unwrap();
    let distances = data.next().unwrap();

    let mut counts = Vec::new();
    for (time, distance) in times.iter().zip(distances) {
        let mut count = 0;
        for t in 0..*time {
            let speed = t;
            let dist = (time - t) * speed;
            if dist > distance {
                count += 1;
            }
        }
        counts.push(count);
    }

    counts.iter().fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input), 288);
    }
}
