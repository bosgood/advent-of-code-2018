use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn code_generator(input: &str) -> Vec<String> {
  input.lines().map(|l| String::from(l)).collect()
}

pub fn part1_count(input: &str) -> (i32, i32) {
  let mut found: HashMap<char, i32> = HashMap::new();

  for c in input.chars().into_iter() {
    let count = match found.get(&c) {
      Some(c) => *c,
      None => 0,
    };
    let _ = found.insert(c, count + 1);
  }

  let mut doubles = 0;
  let mut triples = 0;
  for v in found.values() {
    match v {
      2 => doubles += 1,
      3 => triples += 1,
      _ => {}
    }
  }

  (doubles.min(1), triples.min(1))
}

#[aoc(day2, part1)]
pub fn part1_checksum(input: &[String]) -> i32 {
  let (doubles, triples) = input.iter().fold((0, 0), |memo, line| {
    let (d, t) = part1_count(&line);
    (memo.0 + d, memo.1 + t)
  });
  doubles * triples
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
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_count_example2() {
    let input = "bababc";
    let output = (1, 1);
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_count_example3() {
    let input = "abbcde";
    let output = (1, 0);
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_count_example4() {
    let input = "abcccd";
    let output = (0, 1);
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_count_example5() {
    let input = "aabcdd";
    let output = (1, 0);
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_count_example6() {
    let input = "abcdee";
    let output = (1, 0);
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_count_example7() {
    let input = "ababab";
    let output = (0, 1);
    assert_eq!(part1_count(&input), output);
  }

  #[test]
  fn part1_integration() {
    let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    let output = 12;
    assert_eq!(part1_checksum(&code_generator(&input)), output);
  }
}
