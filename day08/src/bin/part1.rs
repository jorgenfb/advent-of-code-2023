use std::collections::HashMap;

fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

fn process(input: &str) -> u32 {
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

    let mut i = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let (left, right) = nodes.get(current).unwrap();
        let dir = directions[i % directions.len()];

        if dir == 'L' {
            current = left;
        } else {
            current = right;
        }
        i += 1;
    }

    i as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input), 2);
    }
    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input), 6);
    }
}
