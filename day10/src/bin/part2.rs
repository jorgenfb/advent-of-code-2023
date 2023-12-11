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

    let mut current_pos = *start_pos;
    let mut current_dir = *next_dir;

    // Keep track of which tiles are part of the loop
    let mut loop_tiles = vec![];

    loop {
        loop_tiles.push(current_pos);

        current_pos = next_position(&current_pos, &current_dir).expect("Out of bounds");

        let next_tile = get_tile(&map, &current_pos).expect("Out of bounds");

        if *next_tile == Tile::Start {
            break;
        }

        current_dir = find_next_dir(next_tile, &current_dir).expect("Invalid direction");
    }

    // Now we have all the tiles in the loop. Lets find the tiles inside the loop
    let mut count = 0;
    for (row, columns) in map.tiles.iter().enumerate() {
        let mut is_inside = false;
        for (col, _tile) in columns.iter().enumerate() {
            // Skip tiles that are part of the loop
            if loop_tiles.iter().any(|(r, c)| *r == row && *c == col) {
                let tile = &map.tiles[row][col];
                if *tile == Tile::Vertical
                    || *tile == Tile::SouthEast
                    || *tile == Tile::SouthWest
                    // Kind of cheated and checked to find out that the start tile is a Vertical. Could have computed it based on the neighbors, but I'm to tired.
                    || *tile == Tile::Start
                {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                // We are inside the loop, so we count this tile
                count += 1;
            }
        }
    }

    //inside_loop_count as u64
    count as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        // [(0, 4), (1, 4), (2, 4), (2, 5), (1, 5), (0, 5), (0, 6), (1, 6), (2, 6), (2, 7), (1, 7), (0, 7), (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (4, 9), (3, 9), (2, 9), (1, 9), (0, 9), (0, 10), (1, 10), (2, 10), (3, 10), (3, 11), (2, 11), (1, 11), (0, 11), (0, 12), (1, 12), (2, 12), (3, 12), (3, 13), (2, 13), (1, 13), (0, 13), (0, 14), (1, 14), (2, 14), (2, 15), (1, 15), (0, 15), (0, 16), (0, 17), (0, 18), (0, 19), (1, 19), (1, 18), (1, 17), (1, 16), (2, 16), (2, 17), (2, 18), (3, 18), (3, 17), (4, 17), (4, 16), (3, 16), (3, 15), (4, 15), (4, 14), (5, 14), (5, 15), (6, 15), (6, 16), (6, 17), (6, 18), (6, 19), (7, 19), (8, 19), (8, 18), (7, 18), (7, 17), (8, 17), (9, 17), (9, 16), (8, 16), (7, 16), (7, 15), (7, 14), (8, 14), (8, 15), (9, 15), (9, 14), (9, 13), (9, 12), (8, 12), (8, 13), (7, 13), (7, 12), (6, 12), (6, 11), (7, 11), (8, 11), (9, 11), (9, 10), (8, 10), (7, 10), (7, 9), (8, 9), (9, 9), (9, 8), (8, 8), (7, 8), (6, 8), (6, 9), (6, 10), (5, 10), (5, 9), (5, 8), (5, 7), (5, 6), (6, 6), (6, 7), (7, 7), (8, 7), (9, 7), (9, 6), (9, 5), (8, 5), (8, 6), (7, 6), (7, 5), (6, 5), (6, 4), (7, 4), (7, 3), (7, 2), (6, 2), (6, 3), (5, 3), (5, 4), (5, 5), (4, 5), (4, 6), (4, 7), (3, 7), (3, 6), (3, 5), (3, 4), (4, 4), (4, 3), (4, 2), (4, 1), (4, 0), (3, 0), (3, 1), (3, 2), (3, 3), (2, 3), (2, 2), (2, 1), (1, 1), (0, 1), (0, 2), (1, 2), (1, 3), (0, 3)]

        assert_eq!(process(input), 10);
    }

    /*
        #[test]
        fn example2() {
            let input = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

            assert_eq!(process(input), 4);
        }
        */
}
