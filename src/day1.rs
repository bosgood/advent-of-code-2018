#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
  input.lines().fold(0, |memo, l| {
    let int_formatted = l.trim_start_matches('+');
    let val = int_formatted.parse::<i32>().unwrap();
    memo + val
  })
}

#[aoc_generator(day1)]
pub fn number_generator(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|l| {
      let int_formatted = l.trim_start_matches('+');
      int_formatted.parse::<i32>().unwrap()
    }).collect()
}

#[aoc(day1, part1)]
pub fn part1_i32(input: &[i32]) -> i32 {
  input.iter().fold(0, |memo, n| memo + n)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_add2() {
    let input = "-1\n+1";
    let output = 0;
    assert_eq!(part1_chars(input), output);
    assert_eq!(part1_i32(&number_generator(input)), output);
  }

  #[test]
  fn part1_add4() {
    let input = "-1\n+1\n+123123123\n-77039";
    let output = 123046084;
    assert_eq!(part1_chars(input), output);
    assert_eq!(part1_i32(&number_generator(input)), output);
  }
}
