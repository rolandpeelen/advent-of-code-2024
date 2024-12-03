advent_of_code::solution!(2);

#[derive(Eq, PartialEq)]
enum Kind {
    Unsafe,
    Unknown(Option<i32>),
    Increasing(i32),
    Decreasing(i32),
}
use Kind::*;

pub fn within_bounds(a: i32, b: i32) -> bool {
    let diff = (a - b).abs();
    (1..=3).contains(&diff)
}

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines().map(|line| line.split_whitespace());

    let mut safe = 0;

    for line in lines {
        let mut last = Unknown(None);

        for n in line.filter_map(|n| n.parse::<i32>().ok()) {
            match last {
                Unknown(None) => last = Unknown(Some(n)),
                Unknown(Some(old)) if !within_bounds(old, n) => {
                    last = Unsafe;
                    break;
                }
                Unknown(Some(old)) => {
                    if n > old {
                        last = Increasing(n)
                    } else {
                        last = Decreasing(n)
                    }
                }
                Increasing(old) if within_bounds(old, n) && n > old => last = Increasing(n),
                Decreasing(old) if within_bounds(old, n) && n < old => last = Decreasing(n),
                _ => {
                    last = Unsafe;
                    break;
                }
            }
        }

        if last != Unsafe {
            safe += 1
        }
    }

    Some(safe)
}

#[derive(Eq, PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe(xs: &[i32]) -> bool {
    let direction = if xs[0] < xs[1] {
        Direction::Decreasing
    } else {
        Direction::Increasing
    };

    for n in 0..(xs.len() - 1) {
        let nr = xs[n];
        let nl = xs[n + 1];
        let diff = (nr - nl).abs();

        match direction {
            _ if !(1..=3).contains(&diff) => return false,
            Direction::Increasing if nr <= nl => return false,
            Direction::Decreasing if nr >= nl => return false,
            _ => (),
        }
    }

    true
}

fn can_make_safe(xs: &[i32]) -> bool {
    for skip_idx in 0..xs.len() {
        let modified: Vec<i32> = xs
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &n)| n)
            .collect();

        if is_safe(&modified) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<i32> {
    let safe = input
        .lines()
        .filter_map(|line| {
            let line = line
                .split_whitespace()
                .filter_map(|c| c.parse::<i32>().ok())
                .collect::<Vec<_>>();

            if is_safe(&line) || can_make_safe(&line) {
                Some(line)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    safe.len().try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
