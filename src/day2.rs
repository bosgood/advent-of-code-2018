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

pub fn part2_char_difference(one: &str, two: &str) -> i32 {
  let one_b = one.as_bytes();
  let two_b = two.as_bytes();
  let mut count = 0;
  for i in 0..one.len() {
    if one_b[i] != two_b[i] {
      count += 1;
    }
  }
  count
}

pub fn part2_chars_in_common(one: &str, two: &str) -> String {
  let one_b = one.as_bytes();
  let two_b = two.as_bytes();
  let mut common = String::from("");
  for i in 0..one.len() {
    let c = one_b[i];
    if c == two_b[i] {
      common.push(char::from(c));
    }
  }
  common
}

#[aoc(day2, part2)]
pub fn part2_char_difference_closest_strings(input: &[String]) -> String {
  for s1 in input {
    for s2 in input {
      if part2_char_difference(s1, s2) == 1 {
        return part2_chars_in_common(s1, s2);
      }
    }
  }
  String::from("")
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

  /*
    abcde
    fghij
    klmno
    pqrst
    fguij
    axcye
    wvxyz

    The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.
  */

  #[test]
  fn part2_char_difference_1() {
    let input1 = "fghij";
    let input2 = "fguij";
    assert_eq!(part2_char_difference(input1, input2), 1);
  }

  #[test]
  fn part2_char_difference_2() {
    let input1 = "abcde";
    let input2 = "axcye";
    assert_eq!(part2_char_difference(input1, input2), 2);
  }

  #[test]
  fn part2_chars_in_common_1() {
    let input1 = "fghij";
    let input2 = "fguij";
    assert_eq!(part2_chars_in_common(input1, input2), String::from("fgij"));
  }

  #[test]
  fn part2_integration() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
    let output = "fgij";
    assert_eq!(
      part2_char_difference_closest_strings(&code_generator(&input)),
      String::from(output)
    );
  }
}
