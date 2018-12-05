use regex::Regex;

/*
  Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:

  The number of inches between the left edge of the fabric and the left edge of the rectangle.
  The number of inches between the top edge of the fabric and the top edge of the rectangle.
  The width of the rectangle in inches.
  The height of the rectangle in inches.
*/
// FabricSquare expresses the elf's square of fabric in terms of its cartesian coordinates.
// The origin point (0,0) is the top left point.
pub struct FabricSquare {
  id: i32,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
}

impl FabricSquare {
  // Translates from claims to fabric square representations
  // #123 @ 3,2: 5x4
  pub fn new(claim: &str) -> Self {
    lazy_static! {
      static ref re: Regex =
        Regex::new(r"#(?P<id>\d{1,4}) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)")
          .unwrap();
    }
    let cap = re.captures(claim).unwrap();
    FabricSquare {
      id: 0,
      x: 0,
      y: 0,
      width: 0,
      height: 0,
    }
  }

  // Determines if this square overlaps with the given square
  pub fn overlaps_with(&self, square: &FabricSquare) -> bool {
    false
  }

  // Gets the square where this square overlaps with another, if any
  pub fn intersection(&self, square: &FabricSquare) -> Option<FabricSquare> {
    None
  }
}

#[aoc_generator(day3)]
pub fn fabric_square_generator(input: &str) -> Vec<FabricSquare> {
  input.lines().map(|l| FabricSquare::new(l)).collect()
}

#[aoc(day3, part1)]
// If the Elves all proceed with their own plans, none of them will have enough
// fabric. How many square inches of fabric are within two or more claims?
pub fn day3_part1_find_overlapped_area(input: &[FabricSquare]) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  /*
    The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:

    #1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2

    Visually, these claim the following areas:

    ........
    ...2222.
    ...2222.
    .11XX22.
    .11XX22.
    .111133.
    .111133.
    ........

    The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)
  */

  #[test]
  fn part1_overlap_integration_test() {
    let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    let output = 4;
    assert_eq!(
      day3_part1_find_overlapped_area(&fabric_square_generator(&input)),
      output
    );
  }
}
