#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Pair {
    x: u32,
    y: u32,
}

#[aoc_generator(day6, part1)]
pub fn pair_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(", ").collect::<Vec<_>>();
            Pair {
                x: parts[0].parse::<u32>().unwrap(),
                y: parts[1].parse::<u32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn day6_part1(input: &[Pair]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_generator_1() {
        assert_eq!(pair_generator("200, 122")[0], Pair { x: 200, y: 122 });
    }

    #[test]
    fn test_pair_generator_2() {
        assert_eq!(pair_generator("200, 122\n300, 500").len(), 2);
    }

    #[test]
    fn test_day6_part1() {}
}
