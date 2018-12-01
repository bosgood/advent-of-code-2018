#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> i32 {
  input.lines().fold(0, |memo, l| {
    let int_formatted = l.trim_start_matches('+');
    let val = int_formatted.parse::<i32>().unwrap();
    memo + val
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add2() {
    assert_eq!(part1("-1\n+1"), 0);
  }

  #[test]
  fn add4() {
    assert_eq!(part1("-1\n+1\n+123123123\n-77039"), 123046084);
  }
}
