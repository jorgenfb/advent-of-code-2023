fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

fn process(input: &str) -> u32 {
    let res = input
        .lines()
        .filter(|line| line.trim().len() > 0)
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(process(input), 142);
    }
}
