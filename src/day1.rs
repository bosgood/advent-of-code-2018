use std::collections::HashSet;

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

struct State {
  seen: HashSet<i32>,
  current: i32,
}

impl State {
  fn new() -> State {
    let current = 0;
    let mut seen = HashSet::new();
    seen.insert(current);
    State {
      seen: seen,
      current: current,
    }
  }
}

fn part2_process_once(input: &[i32], state: &mut State) -> Option<i32> {
  let res: Result<Vec<i32>, i32> = input
    .iter()
    .map(|n| {
      state.current = state.current + n;
      if state.seen.contains(&state.current) {
        return Err(state.current);
      }
      state.seen.insert(state.current);
      Ok(state.current)
    }).collect();

  match res {
    Ok(_) => None,
    Err(n) => Some(n),
  }
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
  let mut state = State::new();
  loop {
    let res = part2_process_once(input, &mut state);
    match res {
      Some(n) => {
        return n;
      }
      _ => (),
    }
  }
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

  #[test]
  fn part2_example1() {
    assert_eq!(part2(&[1, -1]), 0)
  }

  #[test]
  fn part2_example2() {
    assert_eq!(part2(&[3, 3, 4, -2, -4]), 10)
  }

  #[test]
  fn part2_example3() {
    assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5)
  }

  #[test]
  fn part2_example4() {
    assert_eq!(part2(&[7, 7, -2, -7, -4]), 14)
  }
}
