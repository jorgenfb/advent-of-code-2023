use day03::day03::parse;

fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

fn process(input: &str) -> u32 {
    let (symbols, numbers) = parse(input);

    symbols
        .iter()
        .map(|sym| numbers.iter().filter(|num| num.is_close_by(sym)))
        .filter(|nums| nums.clone().count() == 2)
        .map(|nums| nums.fold(1, |acc, n| acc * n.value))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(process(input), 467835);
    }
}
