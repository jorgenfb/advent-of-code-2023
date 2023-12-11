fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Ground,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

fn parse(input: &str) -> Map {
    let mut start = (0, 0);
    let tiles = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '-' => Tile::Horizontal,
                    '|' => Tile::Vertical,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    'F' => Tile::SouthEast,
                    '7' => Tile::SouthWest,
                    '.' => Tile::Ground,
                    'S' => {
                        start = (row, col);
                        Tile::Start
                    }
                    _ => panic!("Unknown tile: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Map { tiles, start }
}

fn next_position(pos: &(usize, usize), dir: &Dir) -> Option<(usize, usize)> {
    let (row, col) = pos;
    match dir {
        Dir::North => {
            if *row == 0 {
                None
            } else {
                Some((row - 1, *col))
            }
        }
        Dir::South => Some((row + 1, *col)),
        Dir::East => Some((*row, col + 1)),
        Dir::West => {
            if *col == 0 {
                None
            } else {
                Some((*row, col - 1))
            }
        }
    }
}

fn get_tile<'a>(map: &'a Map, pos: &(usize, usize)) -> Option<&'a Tile> {
    let (row, col) = *pos;
    if row >= map.tiles.len() || col >= map.tiles[row].len() {
        None
    } else {
        Some(&map.tiles[row][col])
    }
}

fn find_next_tile<'a>(map: &'a Map, pos: &(usize, usize), dir: &Dir) -> Option<&'a Tile> {
    next_position(pos, dir).and_then(|nx| get_tile(map, &nx))
}

fn find_next_dir(tile: &Tile, dir: &Dir) -> Result<Dir, &'static str> {
    let new_dir = match tile {
        Tile::Horizontal => Ok(*dir),
        Tile::Vertical => Ok(*dir),
        Tile::NorthEast => match dir {
            Dir::North => Err("Cannot move from North into NorthEast"),
            Dir::South => Ok(Dir::East),
            Dir::West => Ok(Dir::North),
            Dir::East => Err("Cannot move from east into NorthEast"),
        },
        Tile::NorthWest => match dir {
            Dir::North => Err("Cannot move from north into NorthWest"),
            Dir::South => Ok(Dir::West),
            Dir::West => Err("Cannot move from east into NorthWest"),
            Dir::East => Ok(Dir::North),
        },
        Tile::SouthEast => match dir {
            Dir::North => Ok(Dir::East),
            Dir::South => Err("Cannot move from north into SouthEast"),
            Dir::West => Ok(Dir::South),
            Dir::East => Err("Cannot move from east into SouthEast"),
        },
        Tile::SouthWest => match dir {
            Dir::South => Err("Cannot move from south into SouthWest"),
            Dir::North => Ok(Dir::West),
            Dir::West => Err("Cannot move from west into SouthWest"),
            Dir::East => Ok(Dir::South),
        },
        Tile::Ground => Err("Cannot move on ground"),
        Tile::Start => Err("Cannot move on start"),
    };

    println!("Tile: {:?}, Dir: {:?}, New dir: {:?}", tile, dir, new_dir);

    new_dir
}

fn process(input: &str) -> u64 {
    let map = parse(input);
    let start_pos = &map.start;

    // Lets find the direction to go in
    let next_dir = [Dir::North, Dir::South, Dir::East, Dir::West]
        .iter()
        .find(|dir| {
            // Get the tile next to the start tile (in given direction)
            let next_tile = find_next_tile(&map, start_pos, dir);

            // If the tile exists, check if we can move in that direction on that tile
            match next_tile {
                Some(tile) => find_next_dir(tile, dir).is_ok(),
                None => return false,
            }
        })
        .expect("Could not find a valid direction to start in");

    let mut steps = 0;
    let mut current_pos = *start_pos;
    let mut current_dir = *next_dir;

    println!("Start: {:?}", current_pos);

    loop {
        steps += 1;

        current_pos = next_position(&current_pos, &current_dir).expect("Out of bounds");

        let next_tile = get_tile(&map, &current_pos).expect("Out of bounds");

        if *next_tile == Tile::Start {
            break;
        }

        current_dir = find_next_dir(next_tile, &current_dir).expect("Invalid direction");
    }

    steps / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(process(input), 8);
    }
}
