use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut lhs, mut rhs): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|str| {
            let split = str.split(" ").collect::<Vec<_>>();
            let a = split.first().unwrap().parse::<i32>().unwrap();
            let b = split.last().unwrap().parse::<i32>().unwrap();

            (a, b)
        })
        .unzip();

    lhs.sort();
    rhs.sort();

    let res: i32 = lhs
        .into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let lhs = input
        .lines()
        .map(|str| {
            let split = str.split(" ").collect::<Vec<_>>();

            let b = split.last().unwrap().parse::<i32>().unwrap();
            map.entry(b).and_modify(|n| *n += 1).or_insert(1);

            split.first().unwrap().parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();

    let res: i32 = lhs
        .into_iter()
        .map(|lhs| lhs * map.get(&lhs).unwrap_or(&0))
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
