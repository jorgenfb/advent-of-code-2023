fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

struct SubSet {
    red: u16,
    green: u16,
    blue: u16,
}

impl SubSet {
    fn from(input: &str) -> SubSet {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for entry in input.trim().split(",") {
            let mut entry_parts = entry.split(" ").filter(|x| !x.trim().is_empty());
            let count = entry_parts.next().unwrap().parse::<u16>().unwrap();
            let color = entry_parts.next().unwrap();

            match color {
                "red" => r += count,
                "green" => g += count,
                "blue" => b += count,
                _ => panic!("Unknown color"),
            }
        }

        SubSet {
            red: r,
            green: g,
            blue: b,
        }
    }
}

fn process(input: &str) -> u16 {
    let res = input
        .lines()
        .filter(|line| line.trim().len() > 0)
        .map(|line| {
            let mut game_parts = line.split(":");

            let game_num = game_parts
                .next()
                .map(|p| &p[5..])
                .map(|p| p.parse::<u16>().unwrap())
                .unwrap();

            for part in game_parts.next().unwrap().split(";") {
                let ss = SubSet::from(part);
                if ss.red > 12 || ss.green > 13 || ss.blue > 14 {
                    return 0;
                }
            }

            return game_num;
        })
        .sum();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_subset() {
        let ss = SubSet::from(" 1 red, 2 green, 6 blue");

        assert_eq!(ss.red, 1);
        assert_eq!(ss.green, 2);
        assert_eq!(ss.blue, 6);
    }

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input), 8);
    }
}
