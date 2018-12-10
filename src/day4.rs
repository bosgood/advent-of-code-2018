use std::num::ParseIntError;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum EventType {
  BeganShift,
  WokeUp,
  FellAsleep,
}

// Event represents an action a guard took
#[derive(Debug, PartialEq, Eq)]
pub struct Event {
  id: i32,
  kind: EventType,
  timestamp: DateTime<Utc>,
}

impl FromStr for Event {
  type Err = ParseIntError;

  /*
    [1518-11-22 23:54] Guard #1237 begins shift
    [1518-11-23 00:04] falls asleep
    [1518-11-23 00:40] wakes up
  */
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref event_re: Regex = Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<content>.+)").unwrap();
      static ref content_re: Regex = Regex::new(r"Guard #(?P<id>\d+) begins shift").unwrap();
    }
    let event_cap = event_re.captures(s).unwrap();
    let year = event_cap
      .name("year")
      .unwrap()
      .as_str()
      .parse::<i32>()
      .unwrap();
    let month = event_cap
      .name("month")
      .unwrap()
      .as_str()
      .parse::<i32>()
      .unwrap();
    let day = event_cap
      .name("day")
      .unwrap()
      .as_str()
      .parse::<i32>()
      .unwrap();
    let hour = event_cap
      .name("hour")
      .unwrap()
      .as_str()
      .parse::<i32>()
      .unwrap();
    let minute = event_cap
      .name("minute")
      .unwrap()
      .as_str()
      .parse::<i32>()
      .unwrap();
    let content = event_cap.name("content").unwrap().as_str();

    let mut kind = EventType::BeganShift;
    let mut id = -1;
    match content {
      "falls asleep" => {
        kind = EventType::FellAsleep;
      }
      "wakes up" => {
        kind = EventType::WokeUp;
      }
      _ => {
        let content_cap = content_re.captures(content).unwrap();
        id = content_cap
          .name("id")
          .unwrap()
          .as_str()
          .parse::<i32>()
          .unwrap();
      }
    };

    Ok(Event {
      id: id,
      kind: kind,
      timestamp: format!("{}-{}-{}T{}:{}:00Z", year, month, day, hour, minute).parse::<DateTime<Utc>>().unwrap(),
    })
  }
}

#[aoc_generator(day4)]
pub fn event_generator(input: &str) -> Vec<Event> {
  let mut cur_id = -1;
  let mut lines: Vec<&str> = input.lines().collect();
  lines.sort();
  lines
    .iter()
    .map(|l| {
      let mut e = l.parse::<Event>().unwrap();
      if e.id == -1 {
        e.id = cur_id;
      } else {
        cur_id = e.id;
      }
      e
    })
    .collect()
}

#[aoc(day4, part1)]
pub fn day4_part1(input: &[Event]) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_event_from_str() {
    let events = event_generator("[1518-06-25 23:58] Guard #1069 begins shift");
    assert_eq!(events.len(), 1);
    assert_eq!(
      events[0],
      Event {
        id: 1069,
        kind: EventType::BeganShift,
        timestamp: "1518-06-25T23:58:00Z".parse::<DateTime<Utc>>().unwrap(),
      }
    );
  }

  #[test]
  fn test_events_from_str() {
    /*
    [1518-11-22 23:54] Guard #1237 begins shift
    [1518-11-23 00:04] falls asleep
    [1518-11-23 00:40] wakes up
    */

    let events = event_generator("[1518-11-23 00:04] falls asleep\n[1518-11-22 23:54] Guard #1237 begins shift\n[1518-11-23 00:40] wakes up");
    assert_eq!(events.len(), 3);
    assert_eq!(
      events[0],
      Event {
        id: 1237,
        kind: EventType::BeganShift,
        timestamp: "1518-11-22T23:54:00Z".parse::<DateTime<Utc>>().unwrap(),
      }
    );
    assert_eq!(
      events[1],
      Event {
        id: 1237,
        kind: EventType::FellAsleep,
        timestamp: "1518-11-23T00:04:00Z".parse::<DateTime<Utc>>().unwrap(),
      }
    );
    assert_eq!(
      events[2],
      Event {
        id: 1237,
        kind: EventType::WokeUp,
        timestamp: "1518-11-23T00:40:00Z".parse::<DateTime<Utc>>().unwrap(),
      }
    );
  }
}
