use day11::process;

fn main() {
    println!("Result: {}\n", process(include_str!("input.txt"), 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(process(input, 1), 374);
    }
}
