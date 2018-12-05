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
#[derive(PartialEq, Eq, Debug)]
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

  // Calculates the area of this square
  pub fn area(&self) -> i32 {
    self.width * self.height
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
  fn claim_to_fabric_square_test1() {
    let input = "#1349 @ 724,871: 21x26";
    let output = FabricSquare {
      id: 1349,
      x: 724,
      y: 871,
      width: 21,
      height: 26,
    };
    assert_eq!(FabricSquare::new(&input), output);
  }

  #[test]
  fn claim_to_fabric_square_test2() {
    let input = "#23 @ 771,152: 16x19";
    let output = FabricSquare {
      id: 23,
      x: 771,
      y: 152,
      width: 16,
      height: 19,
    };
    assert_eq!(FabricSquare::new(&input), output);
  }

  #[test]
  fn claim_to_fabric_square_test3() {
    let input = "#2 @ 102,14: 23x14";
    let output = FabricSquare {
      id: 2,
      x: 102,
      y: 14,
      width: 23,
      height: 14,
    };
    assert_eq!(FabricSquare::new(&input), output);
  }

  #[test]
  fn intersection_not_exists_test1() {
    let s1 = FabricSquare {
      id: 0,
      x: 1,
      y: 3,
      width: 4,
      height: 4,
    };
    let s3 = FabricSquare {
      id: 2,
      x: 5,
      y: 6,
      width: 2,
      height: 2,
    };
    assert_eq!(s1.intersection(&s3).is_none(), true);
  }

  #[test]
  fn intersection_exists_test1() {
    let s1 = FabricSquare {
      id: 0,
      x: 1,
      y: 3,
      width: 4,
      height: 4,
    };
    let s2 = FabricSquare {
      id: 1,
      x: 3,
      y: 1,
      width: 4,
      height: 2,
    };
    let s0 = s1.intersection(&s2).unwrap();
    assert_eq!(s0.x, 3);
    assert_eq!(s0.y, 3);
    assert_eq!(s0.height, 2);
    assert_eq!(s0.width, 2);
  }

  #[test]
  fn area_test1() {
    assert_eq!(
      FabricSquare {
        id: 0,
        x: 0,
        y: 0,
        width: 2,
        height: 2,
      }.area(),
      4
    );
  }

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
