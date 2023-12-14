fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

struct Row {
    data: String,
    known_damaged_count: usize,
    total_damaged_count: usize,
    unknown_indexes: Vec<usize>,
    condition_records: Vec<usize>,
}

// Only allows a row without unknowns
fn computed_damaged_groups(input: &str) -> Vec<usize> {
    input
        .split(".")
        .map(|part| part.len())
        .filter(|count| *count > 0)
        .collect()
}

fn matches_condition_records(input: &str, condition_records: Vec<usize>) -> bool {
    computed_damaged_groups(input) == condition_records
}

fn find_permutations(unknowns: usize, damage_count: usize, path: Vec<bool>) -> Vec<Vec<bool>> {
    // When there are no more unknowns, we're done just return the path
    if unknowns == 0 {
        return vec![path];
    }

    // If ther are no more damaged springs left fill the rest of the path with operational springs
    if damage_count == 0 {
        let mut path = path.clone();
        path.extend(vec![false; unknowns]);
        return vec![path];
    }

    // Otherwise, we have to try both options: Assume the next spring is damaged and assume it's operational
    let mut combined = vec![];

    let mut damaged_path = path.clone();
    damaged_path.push(true);

    combined.extend(find_permutations(
        unknowns - 1,
        damage_count - 1,
        damaged_path,
    ));

    // Only try the operational path if there are enough unknowns left to fill the rest of the path
    if unknowns - 1 >= damage_count {
        let mut operational_path = path.clone();
        operational_path.push(false);

        combined.extend(find_permutations(
            unknowns - 1,
            damage_count,
            operational_path,
        ));
    }

    combined
}

fn count_valid_permutations(row: &Row) -> usize {
    let unknowns = row.unknown_indexes.len();
    let damage_count = row.total_damaged_count - row.known_damaged_count;
    let path = vec![];
    let permutations = find_permutations(unknowns, damage_count, path);

    // Find the valid ones
    let mut valid_count = 0;
    for permutation in permutations {
        let mut i = 0;
        let candidate = row
            .data
            .chars()
            .map(|c| {
                if c == '?' {
                    if permutation[i] {
                        i += 1;
                        return '#';
                    } else {
                        i += 1;
                        return '.';
                    }
                } else {
                    return c;
                }
            })
            .collect::<String>();

        if matches_condition_records(&candidate, row.condition_records.clone()) {
            valid_count += 1;
        }
    }

    valid_count
}

fn parse(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let (pattern_part, records_part) = line.split_once(" ").unwrap();
            let pattern_part = vec![pattern_part; 5].join("?");
            let records_part = vec![records_part; 5].join(",");

            let known_damaged_count = pattern_part.chars().filter(|c| *c == '#').count();

            let unknown_indexes = pattern_part
                .chars()
                .enumerate()
                .filter(|(_i, c)| *c == '?')
                .map(|(i, _c)| i)
                .collect();

            let condition_records = records_part
                .split(",")
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let total_damaged_count = condition_records.iter().sum();

            Row {
                data: pattern_part,
                known_damaged_count,
                total_damaged_count,
                unknown_indexes,
                condition_records,
            }
        })
        .collect()
}

fn process(input: &str) -> u64 {
    let rows = parse(input);

    rows.iter()
        .map(|row| count_valid_permutations(row) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(process(input), 525152);
    }

    //#[test]
    fn test_matches_condition_records() {
        assert!(matches_condition_records("#.#.###", vec![1, 1, 3]));
        assert!(!matches_condition_records("##..###", vec![1, 1, 3]));
    }

    //#[test]
    fn test_count_valid_permutations() {
        let row = Row {
            data: "???.###".to_string(),
            known_damaged_count: 3,
            total_damaged_count: 5,
            unknown_indexes: vec![0, 1, 2],
            condition_records: vec![1, 1, 3],
        };

        assert_eq!(count_valid_permutations(&row), 1);
    }

    //#[test]
    fn test_count_valid_permutations2() {
        let row = Row {
            data: ".??..??...?##.".to_string(),
            known_damaged_count: 2,
            total_damaged_count: 5,
            unknown_indexes: vec![1, 2, 5, 6, 10],
            condition_records: vec![1, 1, 3],
        };

        assert_eq!(count_valid_permutations(&row), 4);
    }
}
