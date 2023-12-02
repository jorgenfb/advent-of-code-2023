fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

fn process(input: &str) -> u32 {
    let res = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|mut line| {
            let mut numbers: Vec<u32> = vec![];

            while line.len() > 0 {
                let first = line.chars().next().unwrap();
                if first.is_digit(10) {
                    numbers.push(first.to_digit(10).unwrap());
                } else if line.starts_with("one") {
                    numbers.push(1);
                } else if line.starts_with("two") {
                    numbers.push(2);
                } else if line.starts_with("three") {
                    numbers.push(3);
                } else if line.starts_with("four") {
                    numbers.push(4);
                } else if line.starts_with("five") {
                    numbers.push(5);
                } else if line.starts_with("six") {
                    numbers.push(6);
                } else if line.starts_with("seven") {
                    numbers.push(7);
                } else if line.starts_with("eight") {
                    numbers.push(8);
                } else if line.starts_with("nine") {
                    numbers.push(9);
                }
                line = &line[1..];
            }
            dbg!(&numbers);

            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(process(input), 281);
    }
}
