#[derive(PartialEq, Clone, Copy)]
enum Observation {
    Empty,
    Galaxy,
}

fn parse(input: &str) -> Vec<Vec<Observation>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Observation::Empty,
                    '#' => Observation::Galaxy,
                    _ => panic!("Unknown tile: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn process(input: &str, expansion_factor: usize) -> usize {
    let observations = parse(input);

    let row_count = observations.len();
    let col_count = observations[0].len();

    let mut positions = vec![];
    for row_id in 0..row_count {
        for col_id in 0..col_count {
            if observations[row_id][col_id] == Observation::Galaxy {
                positions.push((row_id, col_id))
            }
        }
    }

    let empty_rows = observations
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|obs| *obs == Observation::Empty))
        .map(|(row_id, _)| row_id);

    let empty_cols = (0..col_count).filter(|col_id| {
        observations
            .iter()
            .all(|row| row[*col_id] == Observation::Empty)
    });

    // Update the positions by applying the expansion
    for empty_row_id in empty_rows.rev() {
        for position in &mut positions {
            if position.0 > empty_row_id {
                position.0 += expansion_factor;
            }
        }
    }

    for empty_col_id in empty_cols.rev() {
        for position in &mut positions {
            if position.1 > empty_col_id {
                position.1 += expansion_factor;
            }
        }
    }

    let mut total = 0;
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[j];

            total += x1.abs_diff(x2);
            total += y1.abs_diff(y2);
        }
    }

    total
}
