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
    let id = cap.name("id").unwrap().as_str().parse::<i32>().unwrap();
    let x = cap.name("x").unwrap().as_str().parse::<i32>().unwrap();
    let y = cap.name("y").unwrap().as_str().parse::<i32>().unwrap();
    let width = cap.name("width").unwrap().as_str().parse::<i32>().unwrap();
    let height = cap.name("height").unwrap().as_str().parse::<i32>().unwrap();
    FabricSquare {
      id: id,
      x: x,
      y: y,
      width: width,
      height: height,
    }
  }

  // Calculates the area of this square
  pub fn area(&self) -> i32 {
    self.width * self.height
  }

  // Determines if this square overlaps with the given square
  pub fn overlaps_with(&self, square: &FabricSquare) -> bool {
    let x_range1 = Range {
      start: self.x,
      end: self.x + self.width,
    };
    let x_range2 = Range {
      start: square.x,
      end: square.x + square.width,
    };
    let y_range1 = Range {
      start: self.y,
      end: self.y + self.height,
    };
    let y_range2 = Range {
      start: square.y,
      end: square.y + square.height,
    };

    let overlaps_x = x_range1.overlaps(&x_range2) || x_range2.overlaps(&x_range1);
    let overlaps_y = y_range1.overlaps(&y_range2) || y_range2.overlaps(&y_range1);

    overlaps_x && overlaps_y
  }

  // Gets the square where this square overlaps with another, if any
  pub fn intersection(&self, square: &FabricSquare) -> Option<FabricSquare> {
    None
  }
}

struct Range {
  start: i32,
  end: i32,
}

impl Range {
  fn overlaps(&self, r: &Range) -> bool {
    (self.start >= r.start && self.start <= r.end) || (r.start >= self.start && r.start <= self.end)
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

  #[test]
  fn range_overlap_test1() {
    let r1 = Range { start: 0, end: 1 };
    let r2 = Range { start: 2, end: 3 };
    assert_eq!(r1.overlaps(&r2), false);
  }

  #[test]
  fn range_overlap_test2() {
    let r1 = Range { start: 2, end: 3 };
    let r2 = Range { start: 2, end: 3 };
    assert_eq!(r1.overlaps(&r2), true);
  }

  #[test]
  fn range_overlap_test3() {
    let r1 = Range { start: 0, end: 3 };
    let r2 = Range { start: 2, end: 3 };
    assert_eq!(r1.overlaps(&r2), true);
  }

  #[test]
  fn range_overlap_test4() {
    let r1 = Range { start: 0, end: 3 };
    let r2 = Range { start: 50, end: 60 };
    assert_eq!(r1.overlaps(&r2), false);
  }

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
  fn overlap_false_test1() {
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
    assert_eq!(s1.overlaps_with(&s3), false);
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
  fn overlap_true_test1() {
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
    assert_eq!(s1.overlaps_with(&s2), true);
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
