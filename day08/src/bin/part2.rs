use std::collections::HashMap;

fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn process(input: &str) -> u64 {
    let mut lines = input.lines();

    // Parse left / right directions
    let directions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next(); // skip empty line

    // Parse graph
    let nodes: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let source = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (source, (left, right))
        })
        .collect();

    // I did need the help of reddit for this one.
    // Instead of looking actually stepping until all sources ends in Z (which takes forever)
    // we can determine how many steps it takes for each individual path. Then determine their
    // least common multiple. Which will be when they should meet.

    let initial_sources = nodes
        .keys()
        .map(|source| *source)
        .filter(|source| source.ends_with("A"));

    let steps = initial_sources
        .map(|source| {
            let mut current = source;
            let mut i = 0;
            while !current.ends_with("Z") {
                let (left, right) = nodes.get(current).unwrap();
                let dir = directions[i % directions.len()];

                if dir == 'L' {
                    current = left;
                } else {
                    current = right;
                }
                i += 1;
            }
            i
        })
        .collect::<Vec<_>>();

    lcm(&steps) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(process(input), 6);
    }
}
