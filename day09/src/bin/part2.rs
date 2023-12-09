fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            // Build prediction pyramide
            let mut pyramide = vec![nums];
            loop {
                // Generate next row
                let next_row = pyramide
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>();

                let all_zero = next_row.iter().all(|num| *num == 0);
                if all_zero {
                    break;
                }

                pyramide.push(next_row);
            }

            // Predict next number
            let mut pred = 0;
            for row in pyramide.iter().rev() {
                let first_val = row.first().unwrap();
                pred = first_val - pred;
            }
            pred
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(process(input), 2);
    }
}
