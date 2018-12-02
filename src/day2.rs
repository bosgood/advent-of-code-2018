use im::HashSet as ImHashSet;
use std::collections::HashSet;

#[aoc_generator(day2)]
pub fn code_generator(input: &str) -> Vec<String> {
  input.lines().map(|l| String::from(l)).collect()
}

pub fn part1_count(input: &str) -> (i32, i32) {
  (0, 0)
}

#[aoc(day2, part1)]
pub fn part1_checksum(input: &[String]) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  /*

    abcdef contains no letters that appear exactly two or three times.
    bababc contains two a and three b, so it counts for both.
    abbcde contains two b, but no letter appears exactly three times.
    abcccd contains three c, but no letter appears exactly two times.
    aabcdd contains two a and two d, but it only counts once.
    abcdee contains two e.
    ababab contains three a and three b, but it only counts once.

  */

  #[test]
  fn part1_count_example1() {
    let input = "abcdef";
    let output = (0, 0);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_count_example2() {
    let input = "bababc";
    let output = (1, 1);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_count_example3() {
    let input = "abbcde";
    let output = (1, 0);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_count_example4() {
    let input = "abcccd";
    let output = (0, 1);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_count_example5() {
    let input = "aabcdd";
    let output = (1, 0);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_count_example6() {
    let input = "abcdee";
    let output = (1, 0);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_count_example7() {
    let input = "ababab";
    let output = (0, 1);
    assert_eq!(part1_count(input), output);
  }

  #[test]
  fn part1_example() {
    let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    let output = 12;
    assert_eq!(part1_checksum(&code_generator(&input)), output);
  }
}
