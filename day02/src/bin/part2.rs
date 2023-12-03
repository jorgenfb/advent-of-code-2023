fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn from(input: &str) -> Set {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for entry in input.trim().split(",") {
            let mut entry_parts = entry.split(" ").filter(|x| !x.trim().is_empty());
            let count = entry_parts.next().unwrap().parse::<u32>().unwrap();
            let color = entry_parts.next().unwrap();

            match color {
                "red" => r += count,
                "green" => g += count,
                "blue" => b += count,
                _ => panic!("Unknown color"),
            }
        }

        Set {
            red: r,
            green: g,
            blue: b,
        }
    }
}

struct Game {
    sets: Vec<Set>,
}

impl Game {
    fn from(line: &str) -> Game {
        let mut game_parts = line.split(":");
        game_parts.next();

        let sets = game_parts
            .next()
            .unwrap()
            .split(";")
            .map(|s| Set::from(s))
            .collect();

        Game { sets }
    }

    fn min_set(&self) -> Set {
        let r = self.sets.iter().map(|s| s.red).max().unwrap();
        let g = self.sets.iter().map(|s| s.green).max().unwrap();
        let b = self.sets.iter().map(|s| s.blue).max().unwrap();

        Set {
            red: r,
            green: g,
            blue: b,
        }
    }
}

fn process(input: &str) -> u32 {
    let res = input
        .lines()
        .filter(|line| line.trim().len() > 0)
        .map(|line| {
            let ms = Game::from(line).min_set();
            ms.red * ms.blue * ms.green
        })
        .sum();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_subset() {
        let ss = Set::from(" 1 red, 2 green, 6 blue");

        assert_eq!(ss.red, 1);
        assert_eq!(ss.green, 2);
        assert_eq!(ss.blue, 6);
    }

    #[test]
    fn min_set() {
        let ms = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").min_set();
        assert_eq!(ms.red, 4);
        assert_eq!(ms.green, 2);
        assert_eq!(ms.blue, 6);
    }

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input), 2286);
    }
}
