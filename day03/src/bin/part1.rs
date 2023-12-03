use day03::day03::parse;

fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

fn process(input: &str) -> u32 {
    let (symbols, numbers) = parse(input);

    // For each number, figure out if we have a symbol close by
    numbers
        .iter()
        .filter(|num| symbols.iter().any(|sym| num.is_close_by(sym)))
        .map(|num| num.value)
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

        assert_eq!(process(input), 4361);
    }
}
