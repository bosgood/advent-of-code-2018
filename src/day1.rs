use im::HashSet as ImHashSet;
use std::clone::Clone;
use std::collections::HashSet;

#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
  input.lines().fold(0, |memo, l| {
    let int_formatted = l.trim_start_matches('+');
    let val = int_formatted.parse::<i32>().unwrap();
    memo + val
  })
}

#[aoc(day1, part2, inline)]
pub fn part2_chars(input: &str) -> i32 {
  let nums: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
  let mut state = State::new();
  loop {
    let res = part2_process_once(&nums, &mut state);
    match res {
      Some(n) => {
        return n;
      }
      _ => (),
    }
  }
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

struct StateImmutable {
  seen: ImHashSet<i32>,
  current: i32,
  result: Option<i32>,
}

impl StateImmutable {
  fn new() -> StateImmutable {
    let current = 0;
    let seen = ImHashSet::new().update(current);
    StateImmutable {
      seen: seen,
      current: current,
      result: None,
    }
  }

  fn update(&self, val: i32) -> Self {
    let current = self.current + val;
    StateImmutable {
      seen: self.seen.update(current),
      current: current,
      result: self.result,
    }
  }

  fn finish(&self, val: i32) -> Self {
    let next = self.update(val);
    StateImmutable {
      seen: next.seen,
      current: next.current,
      result: Some(next.current),
    }
  }

  fn reached_loop(&self, val: i32) -> bool {
    self.seen.contains(&(self.current + val))
  }
}

impl Clone for StateImmutable {
  fn clone(&self) -> Self {
    StateImmutable {
      seen: self.seen.clone(),
      current: self.current,
      result: self.result,
    }
  }
}

fn part2_process_once_immutable(input: &[i32], initial_state: StateImmutable) -> StateImmutable {
  input
    .iter()
    .fold(initial_state, |memo, n| match memo.result {
      Some(_) => memo,
      None => {
        if memo.reached_loop(*n) {
          return memo.finish(*n);
        }
        memo.update(*n)
      }
    })
}

#[aoc(day1, part2, immutable)]
pub fn part2_immutable(input: &[i32]) -> i32 {
  let mut state = StateImmutable::new();
  loop {
    let next_state = part2_process_once_immutable(input, state);
    match next_state.result {
      Some(n) => {
        return n;
      }
      None => {
        state = next_state;
      }
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
    let input = "+1\n-1";
    let output = 0;
    assert_eq!(part2(&number_generator(input)), output);
    assert_eq!(part2_chars(&input), output);
  }

  #[test]
  fn part2_example1_immutable() {
    let input = "+1\n-1";
    let output = 0;
    assert_eq!(part2_immutable(&number_generator(input)), output);
  }

  #[test]
  fn part2_example2() {
    let input = "+3\n+3\n+4\n-2\n-4";
    let output = 10;
    assert_eq!(part2(&number_generator(input)), output);
    assert_eq!(part2_chars(&input), output);
  }

  #[test]
  fn part2_example2_immutable() {
    let input = "+3\n+3\n+4\n-2\n-4";
    let output = 10;
    assert_eq!(part2_immutable(&number_generator(input)), output);
  }

  #[test]
  fn part2_example3() {
    let input = "-6\n+3\n+8\n+5\n-6";
    let output = 5;
    assert_eq!(part2(&&number_generator(input)), output);
    assert_eq!(part2_chars(&input), output);
  }

  #[test]
  fn part2_example3_immutable() {
    let input = "-6\n+3\n+8\n+5\n-6";
    let output = 5;
    assert_eq!(part2_immutable(&number_generator(input)), output);
  }

  #[test]
  fn part2_example4() {
    let input = "+7\n+7\n-2\n-7\n-4";
    let output = 14;
    assert_eq!(part2(&number_generator(input)), output);
    assert_eq!(part2_chars(&input), output);
  }

  #[test]
  fn part2_example4_immutable() {
    let input = "+7\n+7\n-2\n-7\n-4";
    let output = 14;
    assert_eq!(part2_immutable(&number_generator(input)), output);
  }
}
