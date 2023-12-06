fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

fn bin_search_range(range: (u64, u64), f: impl Fn(u64) -> bool) -> u64 {
    let mut range = range;
    while range.0 < range.1 {
        let mid = (range.0 + range.1) / 2;
        if f(mid) {
            range.1 = mid;
        } else {
            range.0 = mid + 1;
        }
    }
    range.0
}

fn parse(input: &str) -> (u64, u64) {
    let mut data = input.lines().map(|line| {
        line.split_once(" ")
            .map(|(_, line)| line)
            .unwrap()
            .replace(" ", "")
            .parse::<u64>()
            .unwrap()
    });

    let tot_time = data.next().unwrap();
    let max_dist = data.next().unwrap();
    (tot_time, max_dist)
}

fn process(input: &str) -> u64 {
    let (tot_time, max_dist) = parse(input);

    let compute_dist = |speed, tot_time| (tot_time - speed) * speed;

    // The distance function increases until it reaches the halfway point, then decreases. That means that we
    // can use binary search to find the two bounds where the distance is equal to the max distance. We just need
    // to search in both directions (from the middle) to find the two bounds.
    let lower = bin_search_range((0, tot_time), |t| compute_dist(t, tot_time) > max_dist);
    let upper = bin_search_range((0, tot_time), |t| compute_dist(t, tot_time) < max_dist);

    upper - lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input), 71503);
    }
}
